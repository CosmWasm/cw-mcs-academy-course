use cosmwasm_std::{
    ensure, Addr, BankMsg, DepsMut, Env, Event, MessageInfo, Order, Response, StdResult, Uint128,
};

use super::POINTS_SCALE;
use crate::error::ContractError;
use crate::state::{Correction, CORRECTION, DENOM_CORRECTION, MEMBERSHIP, TOTAL_WEIGHT};
use common::state::membership::MEMBERS;

pub fn distribute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let mut resp = Response::new().add_attribute("action", "distribute_tokens");

    let total_weight = TOTAL_WEIGHT.load(deps.storage)?.u128();
    for coin in info.funds {
        let mut correction = DENOM_CORRECTION
            .may_load(deps.storage, &coin.denom)?
            .unwrap_or_default();

        let balance = deps
            .querier
            .query_balance(env.contract.address.clone(), &coin.denom)?
            .amount;
        let amount = balance - correction.withdrawable_total;
        correction.withdrawable_total = amount;
        let points = amount.u128() * POINTS_SCALE + correction.points_leftover.u128();
        let ppw = points / total_weight;
        let distributed = ppw * total_weight;
        correction.points_leftover = (points - distributed).into();
        correction.points_per_weight += Uint128::new(ppw);

        DENOM_CORRECTION.save(deps.storage, &coin.denom, &correction)?;

        let ev = Event::new("token_distribution")
            .add_attribute("denom", coin.denom)
            .add_attribute("amount", amount);
        resp = resp.add_event(ev);
    }

    Ok(resp)
}

pub fn new_member(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    addr: String,
    weight: u64,
) -> Result<Response, ContractError> {
    ensure!(
        info.sender == MEMBERSHIP.load(deps.storage)?,
        ContractError::Unauthorized
    );

    let addr = Addr::unchecked(addr);

    let corrections: Vec<_> = DENOM_CORRECTION
        .range(deps.storage, None, None, Order::Ascending)
        .map(|denom_correction| -> Result<_, ContractError> {
            let (denom, denom_correction) = denom_correction?;
            let points_correction = denom_correction.points_per_weight.u128() * weight as u128;
            let correction = Correction {
                points_correction: -(points_correction as i64),
                withdrawn_funds: Uint128::zero(),
            };
            Ok((denom, correction))
        })
        .collect::<Result<_, _>>()?;

    for (denom, correction) in corrections {
        CORRECTION.save(deps.storage, (&addr, &denom), &correction)?;
    }

    let resp = Response::new()
        .add_attribute("action", "new_member")
        .add_attribute("addr", addr)
        .add_attribute("weight", weight.to_string());

    Ok(resp)
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    weight: u64,
    diff: i64,
) -> Result<Response, ContractError> {
    let membership = MEMBERSHIP.load(deps.storage)?;
    MEMBERS
        .query(&deps.querier, membership, &info.sender)?
        .ok_or(ContractError::Unauthorized)?;

    let funds = deps.querier.query_all_balances(env.contract.address)?;
    let withdraw: Vec<_> = funds
        .into_iter()
        .map(|mut coin| -> Result<_, ContractError> {
            let mut denom_correction = DENOM_CORRECTION
                .may_load(deps.storage, &coin.denom)?
                .unwrap_or_default();
            let mut correction = CORRECTION
                .may_load(deps.storage, (&info.sender, &coin.denom))?
                .unwrap_or_default();

            let points = (denom_correction.points_per_weight.u128() * weight as u128) as i128
                + correction.points_correction as i128;
            let points = points as u128;

            let amount = points / POINTS_SCALE - correction.withdrawn_funds.u128();

            denom_correction.withdrawable_total -= Uint128::new(amount);
            correction.withdrawn_funds += Uint128::new(amount);
            correction.points_correction -= diff * denom_correction.points_per_weight.u128() as i64;

            DENOM_CORRECTION.save(deps.storage, &coin.denom, &denom_correction)?;
            CORRECTION.save(deps.storage, (&info.sender, &coin.denom), &correction)?;

            coin.amount = Uint128::new(amount);

            Ok(coin)
        })
        .collect::<Result<_, _>>()?;

    TOTAL_WEIGHT.update(deps.storage, |weight| -> StdResult<_> {
        Ok(((weight.u128() as i128 + diff as i128) as u128).into())
    })?;

    let events = withdraw.iter().map(|coin| {
        Event::new("withdrawn")
            .add_attribute("denom", coin.denom.clone())
            .add_attribute("amount", coin.amount.to_string())
    });

    let resp = Response::new()
        .add_attribute("action", "withdraw")
        .add_events(events);

    let msg = BankMsg::Send {
        to_address: info.sender.into(),
        amount: withdraw,
    };

    let resp = resp.add_message(msg);
    Ok(resp)
}
