use aura_std::types::smartaccount::v1beta1::{
    MsgActivateAccountRequest, MsgActivateAccountResponse, 
    QueryGenerateAccountRequest, QueryGenerateAccountResponse,
    QueryParamsRequest, QueryParamsResponse,
    MsgRecoverRequest, MsgRecoverResponse
};
use test_tube::runner::result::{RunnerExecuteResult, RunnerResult};
use test_tube::{
    account::{Account, SigningAccount},
    runner::Runner,
};

pub struct SmartAccount<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> super::Module<'a, R> for SmartAccount<'a, R> {
    fn new(runner: &'a R) -> Self {
        SmartAccount { runner }
    }
}

impl<'a, R> SmartAccount<'a, R>
where
    R: Runner<'a>,
{
    pub fn recover(
        &self,
        address: String,
        public_key: aura_std::shim::Any,
        credentials: String,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgRecoverResponse> {
        self.runner.execute(
            MsgRecoverRequest {
                creator: signer.address(),
                address,
                public_key: Some(public_key),
                credentials
            },
            "/aura.smartaccount.v1beta1.MsgRecover",
            signer,
        )
    }

    pub fn activate_account(
        &self,
        code_id: u64,
        salt: Vec<u8>,
        init_msg: Vec<u8>,
        pub_key: aura_std::shim::Any,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgActivateAccountResponse>
    {
        self.runner.execute(
            MsgActivateAccountRequest {
                account_address: signer.address(),
                code_id,
                salt,
                init_msg,
                pub_key: Some(pub_key),
            },
            "/aura.smartaccount.v1beta1.MsgActivateAccount",
            signer,
        )
    }

    pub fn query_generate_account(
        &self, 
        code_id: u64,
        salt: Vec<u8>, 
        init_msg: Vec<u8>,
        public_key: aura_std::shim::Any
    ) -> RunnerResult<String>
    {
        let res = self
            .runner
            .query::<QueryGenerateAccountRequest, QueryGenerateAccountResponse>(
                "/aura.smartaccount.v1beta1.Query/GenerateAccount",
                &QueryGenerateAccountRequest {
                    code_id,
                    salt,
                    init_msg,
                    public_key: Some(public_key),
                },
            )?;

        Ok(res.address)
    }

    pub fn query_params(&self) -> RunnerResult<QueryParamsResponse>
    {
        let res = self
            .runner
            .query::<QueryParamsRequest, QueryParamsResponse>(
                "/aura.smartaccount.v1beta1.Query/Params",
                &QueryParamsRequest {},
            )?;
            
        Ok(res)
    }
}
