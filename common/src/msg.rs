use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

pub mod membership {
    use super::*;

    #[cw_serde]
    pub enum ExecMsg {
        ProposeMember { addr: String },
    }

    #[cw_serde]
    #[derive(QueryResponses)]
    pub enum QueryMsg {
        #[returns(IsMemberResp)]
        IsMember { addr: String },
    }

    #[cw_serde]
    pub struct IsMemberResp {
        pub is_member: bool,
    }
}

#[cw_serde]
pub struct ProposeMemberData {
    pub owner_addr: String,
    pub proxy_addr: String,
}

#[cw_serde]
pub struct WithdrawableResp {
    pub funds: Vec<Coin>,
}
