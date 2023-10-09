#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Timestamp,
    AllBalanceResponse, QueryRequest, BankQuery, QuerierWrapper, Empty, to_binary,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{SudoMsg, InstantiateMsg, QueryMsg, Any, CallInfo};
use crate::state::{
    LIMITS, Limit, BALANCES
};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:spend-limit";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SECOND_PER_HOUR: u64 = 3600;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // new limitation for coin using
    let limit: Limit = Limit { 
        limit: msg.limit.amount, 
        used: Uint128::zero(),
        time_set: env.block.time, 
    };

    LIMITS.save(deps.storage, msg.limit.denom, &limit)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut, 
    env: Env, 
    msg: SudoMsg
) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::PreExecute{ msgs, call_info, is_authz }
        => execute_pre_execute(deps,env,msgs,call_info,is_authz),

        SudoMsg::AfterExecute{ msgs, call_info, is_authz }
        => execute_after_execute(deps,env,msgs,call_info,is_authz)

    }
}

fn check_limit(limit: Limit, amount: Uint128, block_time: Timestamp) -> bool {
 
    // spend limit only available in one hour
    if block_time.minus_seconds(SECOND_PER_HOUR) > limit.time_set {
        return true
    }

    // check if amount of coin used reach limit
    if limit.limit < amount || limit.limit.checked_sub(amount).unwrap() < limit.used {
        return false
    }

    true
}

fn execute_after_execute(
    deps: DepsMut,
    env: Env,
    _msgs: Vec<Any>,
    _call_info: CallInfo,
    _is_authz: bool,
) -> Result<Response, ContractError> {
    let pre_balances = BALANCES.load(deps.storage)?;

    // account after-execute tx balances
    let contract_balance = contract_all_balances(deps.querier,env.contract.address.to_string())?;
    let after_balances = contract_balance.amount;

    for pre_balance in pre_balances {
        // if has spendlimit for denom
        if let Some(mut limit) = LIMITS.may_load(deps.storage, pre_balance.denom.clone())? {
            let matching_coin = after_balances.iter().find(|fund| fund.denom.eq(&pre_balance.denom));
            let amount = match matching_coin {
                Some(coin) => coin.amount,
                None => Uint128::zero()
            };

            // check if coin with denom has been used
            // this for demo only, 
            // in real case, user can cheat here by including withdrawal message and deposit message in the same tx
            if pre_balance.amount > amount {
                // used amount
                let used_amount = pre_balance.amount.checked_sub(amount).unwrap();
                // check if spendlimit has been reach
                if !check_limit(limit.clone(), used_amount, env.block.time) {
                    return Err(ContractError::CustomError {
                        val: format!("limit exceed for denom: {}", pre_balance.denom)
                    })
                }

                // update used amount
                limit.used = limit.used.checked_add(used_amount).unwrap();
                LIMITS.save(deps.storage, pre_balance.denom, &limit)?;
            }
        }
    }

    Ok(Response::new().add_attribute("action", "after_execute"))
}

fn execute_pre_execute(
    deps: DepsMut,
    env: Env,
    _msgs: Vec<Any>,
    _call_info: CallInfo,
    _is_authz: bool,
) -> Result<Response, ContractError> {
    // get the balances of contract
    let contract_balance = contract_all_balances(deps.querier, env.contract.address.to_string())?;

    // account pre-execute tx balances  
    BALANCES.save(deps.storage, &contract_balance.amount)?;

    Ok(Response::new().add_attribute("action", "pre_execute"))
}

fn contract_all_balances<'a>(querier: QuerierWrapper<'a, Empty>, address: String) -> StdResult<AllBalanceResponse> {
    querier.query(&QueryRequest::Bank(BankQuery::AllBalances {
        address
    }))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLimt { denom } => to_binary(&query_limit(deps, env, denom)?)
    }
}

pub fn query_limit(
    deps: Deps,
    _env: Env,
    denom: String
) -> StdResult<Option<Limit>> {
    LIMITS.may_load(deps.storage, denom)
}