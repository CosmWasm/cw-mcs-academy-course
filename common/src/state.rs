pub mod membership {
    use cosmwasm_std::Addr;
    use cw_storage_plus::Map;

    pub const MEMBERS: Map<&Addr, cosmwasm_std::Empty> = Map::new("members");
}
