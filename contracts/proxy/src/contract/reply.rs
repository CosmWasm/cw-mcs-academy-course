use cosmwasm_std::{coins, BankMsg, DepsMut, Env, Response};

use crate::error::ContractError;
use crate::state::{CONFIG, PENDING_WITHDRAWAL};

pub fn withdraw(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let withdrawal_info = PENDING_WITHDRAWAL.load(deps.storage)?;

    let config = CONFIG.load(deps.storage)?;
    let total_amount = deps
        .querier
        .query_balance(env.contract.address, &config.denom)?;

    let amount = withdrawal_info.amount.unwrap_or(total_amount.amount);

    let send_msg = BankMsg::Send {
        to_address: withdrawal_info.receiver.into_string(),
        amount: coins(amount.u128(), &config.denom),
    };

    let resp = Response::new()
        .add_message(send_msg)
        .add_attribute("amount", amount.to_string());

    Ok(resp)
}
