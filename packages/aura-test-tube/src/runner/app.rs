use cosmrs::Any;
use cosmwasm_std::Coin;
use prost::Message;
use test_tube::account::SigningAccount;
use test_tube::runner::result::{RunnerExecuteResult, RunnerResult};
use test_tube::runner::Runner;
use test_tube::BaseApp;

const FEE_DENOM: &str = "uaura";
const CHAIN_ID: &str = "aura-testnet";
const DEFAULT_GAS_ADJUSTMENT: f64 = 2.0;
const DEFAULT_GAS_LIMIT: u64 = 20000000;

#[derive(Debug, PartialEq)]
pub struct AuraTestApp {
    inner: BaseApp,
}

impl Default for AuraTestApp {
    fn default() -> Self {
        AuraTestApp::new()
    }
}

impl AuraTestApp {
    pub fn new() -> Self {
        Self {
            inner: BaseApp::new(FEE_DENOM, CHAIN_ID, DEFAULT_GAS_ADJUSTMENT, DEFAULT_GAS_LIMIT),
        }
    }

    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
    pub fn init_base_account(&self, coins: &[Coin]) -> RunnerResult<SigningAccount> {
        self.inner.init_base_account(coins)
    }
    /// Convinience function to create multiple accounts with the same
    /// Initial coins balance
    pub fn init_base_accounts(&self, coins: &[Coin], count: u64) -> RunnerResult<Vec<SigningAccount>> {
        self.inner.init_base_accounts(coins, count)
    }

    pub fn init_local_smart_account(&self, address: String, private_key: Vec<u8>) -> RunnerResult<SigningAccount> {
        self.inner.init_local_smart_account(address, private_key)
    }

    /// Simulate transaction execution and return gas info
    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        self.inner.simulate_tx(msgs, signer)
    }

    /// Set parameter set for a given subspace.
    pub fn set_param_set(&self, subspace: &str, pset: Any) -> RunnerResult<()> {
        self.inner.set_param_set(subspace, pset)
    }

    /// Get parameter set for a given subspace.
    pub fn get_param_set<P: Message + Default>(
        &self,
        subspace: &str,
        type_url: &str,
    ) -> RunnerResult<P> {
        self.inner.get_param_set(subspace, type_url)
    }
}

impl<'a> Runner<'a> for AuraTestApp {
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.inner.execute_multiple(msgs, signer)
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.inner.query(path, q)
    }
}

#[cfg(test)]
mod tests {
    use std::option::Option::None;
    use cosmwasm_std::coins;
    use crate::runner::app::AuraTestApp;
    use test_tube::account::Account;
    use test_tube::runner::*;

    use cosmrs::proto::cosmos::bank::v1beta1::{
        QueryAllBalancesRequest, QueryAllBalancesResponse
    };

    #[test] 
    fn test_query() {
        let app = AuraTestApp::default();

        let acc = app.init_base_account(&coins(100_000_000_000, "uaura")).unwrap();
        let addr = acc.address();

        let acc_balance = get_account_balances(&app, addr, "uaura");

        assert_eq!(acc_balance, 100_000_000_000u128);
    }

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
}