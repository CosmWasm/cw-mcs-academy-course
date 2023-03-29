use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ProposeMemberData {
    pub owner_addr: String,
    pub proxy_addr: String,
}
