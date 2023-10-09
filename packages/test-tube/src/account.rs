use cosmrs::{
    crypto::{secp256k1::SigningKey, PublicKey},
    AccountId,
};
use cosmwasm_std::Coin;

pub const ADDRESS_PREFIX: &str = "aura";

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

impl SigningAccount {
    pub fn new(address: String, signing_key: SigningKey, private_key: Vec<u8>, fee_setting: FeeSetting) -> Self {
        SigningAccount {
            address,
            signing_key,
            private_key,
            fee_setting,
        }
    }

    pub fn fee_setting(&self) -> &FeeSetting {
        &self.fee_setting
    }

    pub fn with_fee_setting(self, fee_setting: FeeSetting) -> Self {
        Self {
            address: self.address,
            signing_key: self.signing_key,
            private_key: self.private_key,
            fee_setting,
        }
    }
}

impl Account for SigningAccount {
    fn public_key(&self) -> PublicKey {
        self.signing_key.public_key()
    }
    fn private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }
    fn address(&self) -> String {
        self.address.clone()
    }
}

impl SigningAccount {
    pub fn signing_key(&'_ self) -> &'_ SigningKey {
        &self.signing_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonSigningAccount {
    public_key: PublicKey,
}

impl From<PublicKey> for NonSigningAccount {
    fn from(public_key: PublicKey) -> Self {
        NonSigningAccount { public_key }
    }
}
impl From<SigningAccount> for NonSigningAccount {
    fn from(signing_account: SigningAccount) -> Self {
        NonSigningAccount {
            public_key: signing_account.public_key(),
        }
    }
}

impl Account for NonSigningAccount {
    fn public_key(&self) -> PublicKey {
        self.public_key
    }
    fn private_key(&self) -> Vec<u8> {
        Vec::<u8>::new()
    }
    fn address(&self) -> String {
        self.account_id().to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeeSetting {
    Auto {
        gas_price: f64,
        gas_adjustment: f64,
    },
    Custom {
        amount: Coin,
        gas_limit: u64,
    },
}
