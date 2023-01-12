use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Empty};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub proxy_code_id: u64,
    pub distribution_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const MEMBERS: Map<Addr, Empty> = Map::new("members");
pub const PROPOSALS: Map<Addr, u64> = Map::new("proposals");
