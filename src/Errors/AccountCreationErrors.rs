use thiserror::Error;


#[derive(Error, Debug)]
pub enum AccountCreationErrors{
    #[error("The user already exists")]
    AlreadyExists(),

    #[error("Internal Error")]
    InternalError(),
}