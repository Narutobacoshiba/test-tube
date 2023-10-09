use cosmwasm_schema::write_api;

use spend_limit::msg::{SudoMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        migrate: MigrateMsg,
        sudo: SudoMsg,
        query: QueryMsg,
    }
}
