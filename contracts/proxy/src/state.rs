use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub denom: String,
    pub direct_part: Decimal,
    pub distribution_contract: Addr,
    pub membership_contract: Addr,
    pub is_closed: bool,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const WEIGHT: Item<u64> = Item::new("weight");
pub const DONATIONS: Item<u64> = Item::new("donations");
pub const CONFIG: Item<Config> = Item::new("config");
pub const HALFTIME: Item<u64> = Item::new("halftime");
pub const LAST_UPDATED: Item<u64> = Item::new("last_updated");
