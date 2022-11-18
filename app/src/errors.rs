use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractCallError {
    #[error("Wrong input, {0}")]
    InputError(String),
    #[error("Function call fail, {0}")]
    CallFail(String),
    #[error("Fail to parse result")]
    ResultParseFail,
    #[error("unknown error")]
    Unknown,
}
