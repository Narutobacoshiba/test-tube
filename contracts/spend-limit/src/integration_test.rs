#[cfg(test)]
mod unit_tests {
    use std::option::Option::None;
    use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
    use cosmos_sdk_proto::traits::MessageExt;
    use cosmos_sdk_proto::cosmos::bank::v1beta1::{MsgSend, MsgSendResponse};
    use cosmos_sdk_proto::cosmos::authz::v1beta1::{MsgGrant, MsgExec, Grant, GenericAuthorization, MsgGrantResponse};
    use cosmwasm_schema::cw_serde;
    use test_tube::runner::result::RunnerExecuteResult;
    use aura_std::types::smartaccount::v1beta1::{Params, CodeID};
    use cosmwasm_std::coins;
    use aura_test_tube::{Wasm, AuraTestApp, SmartAccount};
    use test_tube::{Account, Runner, Module, SigningAccount};
    use cosmrs::proto::cosmos::bank::v1beta1::{
        QueryAllBalancesRequest, QueryAllBalancesResponse
    };

    fn get_account_balances(app: &AuraTestApp, address: String, denom: &str) -> u128 {
        let acc_balance = app.query::<QueryAllBalancesRequest,QueryAllBalancesResponse>(
            "/cosmos.bank.v1beta1.Query/AllBalances",
            &QueryAllBalancesRequest {
                address,
                pagination: None,
            },
        )
        .unwrap()
        .balances
        .into_iter()
        .find(|c| c.denom == denom)
        .unwrap()
        .amount
        .parse::<u128>()
        .unwrap();
        return acc_balance;
    }

    fn send_coin(app: &AuraTestApp, from: &SigningAccount, to: String, amounts: Vec<Coin>) -> RunnerExecuteResult<MsgSendResponse> {
        app.execute(
            MsgSend {
                from_address: from.address(),
                to_address: to,
                amount: amounts,
            }, 
            "/cosmos.bank.v1beta1.MsgSend", 
            from
        )
    }

    #[cw_serde]
    struct EmptyInit {}

    #[cw_serde]
    struct Listen {
        listen: EmptyInit
    }

    #[test]
    fn test_smartaccount() {
        // default chain
        // id: aura-testnet
        // denom: uaura
        let app = AuraTestApp::default();

        let acc = app.init_base_account(&coins(100_000_000_000, "uaura")).unwrap();
        let acc_balance = get_account_balances(&app, acc.address(), "uaura");
        assert_eq!(acc_balance, 100_000_000_000u128);
        
        let wasm = Wasm::new(&app);
        let smartaccount = SmartAccount::new(&app);
        
        let test_code = std::fs::read("../../artifacts/spend_limit.wasm").unwrap(); // load contract wasm 

        // store wasm for smartaccount initialization
        let test_code_id = wasm
            .store_code(
                &test_code, 
                None, 
                &acc  
            )
            .unwrap()
            .data
            .code_id; 
        assert_eq!(test_code_id, 1);
        
        // set whitelist for code id, don't need government
        let params = Params {
            whitelist_code_id: vec![CodeID{
                code_id: test_code_id,
                status: true
            }],
            disable_msgs_list: vec![],
            max_gas_execute: 2000000,
        };
        let param_set = aura_std::shim::Any{
            type_url: String::from("/aura.smartaccount.v1beta1.Params"),
            value: params.to_bytes().unwrap()
        };
        let _ = app.set_param_set("smartaccount", param_set.into()).unwrap();
        // query smartaccount module param set
        let sa_params = smartaccount.query_params().unwrap();
        assert_eq!(sa_params.params, Some(params));

        
        // generate smartaccount address
        let pub_key = aura_std::shim::Any {
            type_url: String::from("/cosmos.crypto.secp256k1.PubKey"),
            value: cosmos_sdk_proto::cosmos::crypto::secp256k1::PubKey { 
                key: acc.public_key().to_bytes()
            }.to_bytes().unwrap()
        };
        // or simple
        // let pub_key = acc.public_key().to_any().unwrap();
        let salt = "test account".as_bytes().to_vec();
        let init_msg = "{\"limit\":{\"denom\":\"uaura\",\"amount\":\"10000\"}}".as_bytes().to_vec();

        let sa_addr = smartaccount.query_generate_account(
            test_code_id, 
            salt.clone(), 
            init_msg.clone(), 
            pub_key.clone()
        ).unwrap();

        // send coin to smartaccount
        let banksend_res: RunnerExecuteResult<MsgSendResponse> = send_coin(
            &app,
        &acc,
        sa_addr.clone(),
        vec![Coin{denom: "uaura".to_string(),amount: "10000000".to_string(),}]
        );       
        assert!(banksend_res.is_ok());
        let acc_balance = get_account_balances(&app, sa_addr.clone(), "uaura");
        assert_eq!(acc_balance, 10000000u128);
        
        // local account which has not been initialized on-chain
        let sa_acc = app.init_local_smart_account(sa_addr.clone(), acc.private_key()).unwrap();
        // initializ smartaccount on-chain
        // execute with default gas setting
        // gas: 2000000
        // gas_price: 0.025
        // in order to modify gas setting, use &sa_acc.with_fee_setting(fee_setting) instead
        let _ = smartaccount.activate_account(
            test_code_id, 
            salt, 
            init_msg, 
            pub_key, 
            &sa_acc,
        ).unwrap();

        let acc2 = app.init_base_account(&coins(10, "uaura")).unwrap();

        let generic_auth = aura_std::shim::Any {
            type_url: String::from("/cosmos.authz.v1beta1.GenericAuthorization"),
            value: GenericAuthorization{
                msg: String::from("/cosmos.bank.v1beta1.MsgSend")
            }.to_bytes().unwrap()
        };
        let ret = app.execute::<MsgGrant, MsgGrantResponse>(
            MsgGrant {
                granter: sa_acc.address(),
                grantee: acc2.address(),
                grant: Some(Grant{
                    authorization: Some(generic_auth.into()),
                    expiration: None
                })
            }, 
            "/cosmos.authz.v1beta1.MsgGrant", 
            &sa_acc
        );
        assert!(ret.is_ok());

    }
}