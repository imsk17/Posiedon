use thiserror::Error;
#[derive(Error, Debug)]
pub enum BlockSerErrorKind {
    #[error("failed to serialize the block from bytes: {0}")]
    FailedToSerializeErrorBlock(String),
    #[error("failed to deserialize the block from bytes: {0}")]
    FailedToDeserializeErrorBlock(String),
}