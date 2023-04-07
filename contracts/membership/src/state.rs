use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Empty};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, UniqueIndex};

#[cw_serde]
pub struct Config {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub proxy_code_id: u64,
    pub distribution_contract: Addr,
    pub minimal_acceptances: u64,
}

pub struct MembersIndexes<'a> {
    pub owner: UniqueIndex<'a, Addr, Addr, Addr>,
}

impl<'a> IndexList<Addr> for MembersIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Addr>> + '_> {
        let v: [&dyn Index<Addr>; 1] = [&self.owner];
        Box::new(v.into_iter())
    }
}

// proxy => owner
//
// secondary indexes:
// * owner
pub fn members() -> IndexedMap<'static, &'static Addr, Addr, MembersIndexes<'static>> {
    let indexes = MembersIndexes {
        owner: UniqueIndex::new(|owner| owner.clone(), "members__owner"),
    };
    IndexedMap::new("members", indexes)
}

pub const CONFIG: Item<Config> = Item::new("config");
// (member_voter, candidate)
pub const VOTES: Map<(&Addr, &Addr), Empty> = Map::new("votes");
pub const PROPOSALS: Map<&Addr, u64> = Map::new("proposals");

pub const AWAITING_INITIAL_RESPS: Item<u64> = Item::new("awaiting_initial_resps");
