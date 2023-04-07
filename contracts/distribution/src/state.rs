use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
#[derive(Default)]
pub struct DenomCorrection {
    pub points_per_weight: Uint128,
    pub points_leftover: Uint128,
    pub withdrawable_total: Uint128,
}

#[cw_serde]
#[derive(Default)]
pub struct Correction {
    pub points_correction: i64,
    pub withdrawn_funds: Uint128,
}

pub const TOTAL_WEIGHT: Item<Uint128> = Item::new("total");
pub const MEMBERSHIP: Item<Addr> = Item::new("membership");
pub const DENOM_CORRECTION: Map<&str, DenomCorrection> = Map::new("denom_correction");
pub const CORRECTION: Map<(&Addr, &str), Correction> = Map::new("correction");
