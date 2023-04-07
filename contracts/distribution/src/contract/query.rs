use common::msg::WithdrawableResp;
use cosmwasm_std::{Addr, Deps, Env, StdResult};

use crate::state::{CORRECTION, DENOM_CORRECTION};

use super::POINTS_SCALE;

pub fn withdrawable(
    deps: Deps,
    env: Env,
    proxy: String,
    weight: u64,
) -> StdResult<WithdrawableResp> {
    let funds = deps.querier.query_all_balances(env.contract.address)?;
    let proxy = Addr::unchecked(proxy);
    let funds: Vec<_> = funds
        .into_iter()
        .map(|mut coin| -> StdResult<_> {
            let denom_correction = DENOM_CORRECTION
                .may_load(deps.storage, &coin.denom)?
                .unwrap_or_default();
            let correction = CORRECTION
                .may_load(deps.storage, (&proxy, &coin.denom))?
                .unwrap_or_default();

            let points = (denom_correction.points_per_weight.u128() * weight as u128) as i128
                + correction.points_correction as i128;
            let points = points as u128;

            coin.amount = (points / POINTS_SCALE - correction.withdrawn_funds.u128()).into();

            Ok(coin)
        })
        .filter(|coin| {
            coin.as_ref()
                .map(|coin| !coin.amount.is_zero())
                .unwrap_or(true)
        })
        .collect::<Result<_, _>>()?;

    Ok(WithdrawableResp { funds })
}
