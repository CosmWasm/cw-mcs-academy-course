use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Direct part has to be between 0 and 1")]
    InalidDirectPart,
    #[error("Unauthorized")]
    Unauthorized,
}
