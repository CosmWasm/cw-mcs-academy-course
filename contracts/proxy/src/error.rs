use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{0}")]
    PaymentError(#[from] PaymentError),
    #[error("Direct part has to be between 0 and 1")]
    InalidDirectPart,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unrecognized reply id: {0}")]
    UnrecognizedReplyId(u64),
}
