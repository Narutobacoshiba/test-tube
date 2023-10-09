use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Limit;
use cosmwasm_std::Coin;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub limit: Coin
}

// A data structure representing an account's message 
// will be passed into the call to the smart-account contract every time tx arrives
#[cw_serde]
pub struct Any {
    pub type_url: String, // url type of message
    pub value:    String, // value of message
    // etc.
    //  MsgData {
    //      type_url: "/cosmos.bank.v1beta1.MsgSend",
    //      value: "{fromAddress:\"aura172r4c7mng5y6ccfqp5klwyulshx6dh2mmd2r0xnmsgugaa754kws8u96pq\",toAddress:\"aura1y3u4ht0p69gz757myr3l0fttchhw3fj2gpeznd\",amount:[{denom:\"uaura\",amount:\"200\"}]}"
    //  }
}

#[cw_serde]
pub struct CallInfo {
    pub fee: Vec<Coin>,
    pub gas: u64,
    pub fee_payer: String,
    pub fee_granter: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum SudoMsg {
    // required `PreExecute` method
    PreExecute{
        //list of messages in transaction 
        msgs: Vec<Any>,
        call_info: CallInfo,
        is_authz: bool,
    },

    // required `AfterExecute` method
    AfterExecute{
        //list of messages in transaction 
        msgs: Vec<Any>,
        call_info: CallInfo,
        is_authz: bool,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

// struct for message with type `/cosmos.bank.v1beta1.MsgSend`
// it's same as message struct in cosmos-sdk or cosmjs
#[cw_serde]
pub struct MsgSend {
    pub from_address: String, // sender
    pub to_address: String, // receiver
    pub amount: Vec<Coin>, // amount
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Option<Limit>)]
    GetLimt {
        denom: String
    },
}

