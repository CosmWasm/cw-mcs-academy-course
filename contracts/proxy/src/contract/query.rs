use common::msg::WithdrawableResp;
use cosmwasm_std::{Deps, Env, StdResult};
use distribution::msg::QueryMsg as DistributionQueryMsg;

use crate::state::{CONFIG, WEIGHT};

pub fn withdrawable(deps: Deps, env: Env) -> StdResult<WithdrawableResp> {
    let config = CONFIG.load(deps.storage)?;
    let weight = WEIGHT.load(deps.storage)?;

    let mut resp: WithdrawableResp = deps.querier.query_wasm_smart(
        config.distribution_contract,
        &DistributionQueryMsg::Withdrawable {
            proxy: env.contract.address.to_string(),
            weight,
        },
    )?;

    let funds = deps.querier.query_all_balances(env.contract.address)?;

    for coin in funds {
        let idx = resp.funds.iter().position(|c| coin.denom == c.denom);
        match idx {
            Some(idx) => resp.funds[idx].amount += coin.amount,
            None => resp.funds.push(coin),
        }
    }

    Ok(resp)
}
