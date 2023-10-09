# Aura Integrantion Test

example in file **./contracts/spend-limt/src/integration_test.rs**

## App

use **app** to interact with actual logic of Aura chain

```Rust
// default chain
// id: aura-testnet
// denom: uaura
let app = AuraTestApp::default();
```

### Methods

* `Execute message`
    ```Rust
    fn execute<M, R>(
        &self,
        msg: M,
        type_url: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    ```
* `Execute multi message`
    ```Rust
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    ```
* `Query`
    ```Rust
    fn query<Q, R>(
        &self, 
        path: &str, 
        q: &Q
    ) -> RunnerResult<R>

    /******** Example ********/

    let acc_balance = app.query::<QueryAllBalancesRequest,QueryAllBalancesResponse>(
        "/cosmos.bank.v1beta1.Query/AllBalances",
        &QueryAllBalancesRequest {
            address,
            pagination: None,
        },
    ).unwrap()
    .balances
    .into_iter()
    .find(|c| c.denom == denom)
    .unwrap()
    .amount
    .parse::<u128>()
    .unwrap();
    ```
* `Init base account on-chain`
    ```Rust
    pub fn init_base_account(
        &self, 
        coins: &[Coin]
    ) -> RunnerResult<SigningAccount>

    /******** Example ********/

    // init new base account with 100_000_000_000uaura, account will exist on-chain
    let acc = app.init_base_account(&coins(100_000_000_000, "uaura")).unwrap();
    ```
* `Init local smart account`
    ```Rust
    pub fn init_local_smart_account(
        &self, 
        address: String, 
        private_key: Vec<u8>
    ) -> RunnerResult<SigningAccount>

    /******** Example ********/

    // local account which has not been initialized on-chain
    let sa_acc = app.init_local_smart_account(sa_addr.clone(), acc.private_key()).unwrap();
    ```
* `Simulate`
    ```Rust
    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<GasInfo>
    ```
* `Set Params`
    ```Rust
    /// Set parameter set for a given subspace.
    pub fn set_param_set(
        &self, 
        subspace: &str, 
        pset: Any
    ) -> RunnerResult<()>
    ```
* `Get Params`
    ```Rust
    /// Get parameter set for a given subspace.
    pub fn get_param_set<P: Message + Default>(
        &self,
        subspace: &str,
        type_url: &str,
    ) -> RunnerResult<P>
    ```

## Wasm

use **wasm** to interact with wasmd module

```Rust
let app = AuraTestApp::default();
let wasm = Wasm::new(&app);
```

### Methods

* `Store Code`
    ```Rust
     pub fn store_code(
        &self,
        wasm_byte_code: &[u8],
        instantiate_permission: Option<AccessConfig>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgStoreCodeResponse>


    /******** Example ********/

    let app = AuraTestApp::default();
    let wasm = Wasm::new(&app);

    let acc = app.init_base_account(&coins(100_000_000_000, "uaura")).unwrap();

    // load contract wasm 
    let wasm_code = std::fs::read("../../artifacts/spend_limit.wasm").unwrap(); 

    // store wasm for smartaccount initialization
    let code_id = wasm.store_code(
        &test_code, 
        None, 
        &acc  
    ).unwrap()
    ```
* `Instantiate`
    ```Rust
    pub fn instantiate<M>(
        &self,
        code_id: u64,
        msg: &M,
        admin: Option<&str>,
        label: Option<&str>,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgInstantiateContractResponse>
    ```
* `Instantiate2`
    ```Rust
    pub fn instantiate2<M>(
        &self,
        code_id: u64,
        msg: &M,
        admin: Option<&str>,
        label: Option<&str>,
        funds: &[Coin],
        salt: Vec<u8>,
        fix_msg: bool,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgInstantiateContract2Response>
    ```
* `Execute`
    ```Rust
    pub fn execute<M>(
        &self,
        contract: &str,
        msg: &M,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse>
    ```
* `Query`
    ```Rust
    pub fn query<M, Res>(
        &self, 
        contract: &str, 
        msg: &M
    ) -> RunnerResult<Res>
    ```

## SmartAccount 

use **smartaccount** to interact with smart-account module

```Rust
let app = AuraTestApp::default();
let smartaccount = SmartAccount::new(&app);
```

### Methods

* `Recover`
    ```Rust
    pub fn recover(
        &self,
        address: String,
        public_key: aura_std::shim::Any,
        credentials: String,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgRecoverResponse>
    ```
* `Activate account`
    ```Rust
    pub fn activate_account(
        &self,
        code_id: u64,
        salt: Vec<u8>,
        init_msg: Vec<u8>,
        pub_key: aura_std::shim::Any,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgActivateAccountResponse>
    ```
* `Query generate smart account`
    ```Rust
    pub fn query_generate_account(
        &self, 
        code_id: u64,
        salt: Vec<u8>, 
        init_msg: Vec<u8>,
        public_key: aura_std::shim::Any
    ) -> RunnerResult<String>
    ```
* `Query params`
    ```Rust
    pub fn query_params(&self) -> RunnerResult<QueryParamsResponse>
    ```

## SigningAccount

signer account

```Rust
pub trait Account {
    fn public_key(&self) -> PublicKey;
    fn private_key(&self) -> Vec<u8>;
    fn address(&self) -> String;
    fn account_id(&self) -> AccountId {
        self.public_key()
            .account_id(ADDRESS_PREFIX)
            .expect("ADDRESS_PREFIX is constant and must valid")
    }
}
pub struct SigningAccount {
    address: String,
    signing_key: SigningKey,
    private_key: Vec<u8>,
    fee_setting: FeeSetting,
}
```

### Methods

* `New`
    ```Rust
    pub fn new(address: String, signing_key: SigningKey, private_key: Vec<u8>, fee_setting: FeeSetting) -> Self
    ```
* `With fee setting`
    ```Rust
    pub fn with_fee_setting(self, fee_setting: FeeSetting) -> Self

    /******** Example ********/

    // local account which has not been initialized on-chain
    let sa_acc = app.init_local_smart_account(sa_addr.clone(), acc.private_key()).unwrap();
    // initializ smartaccount on-chain
    // execute with default gas setting
    // gas: 2000000
    // gas_price: 0.025
    let _ = smartaccount.activate_account(
        test_code_id, 
        salt, 
        init_msg, 
        pub_key, 
        &sa_acc,
    ).unwrap();


    let fee_setting = FeeSetting::Custom { 
        amount: Coin { 
            denom: String::from("uaura"), 
            amount: Uint128::from(100000u128)
        }, 
        gas_limit: 2000000u64 
    }
    // initializ smartaccount on-chain
    // execute with custom gas setting
    // gas: 2000000
    // gas_price: 0.05
    let _ = smartaccount.activate_account(
        test_code_id, 
        salt, 
        init_msg, 
        pub_key, 
        &sa_acc.with_fee_setting(fee_setting),
    ).unwrap();
    ```

