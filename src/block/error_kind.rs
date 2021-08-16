#[derive(Debug)]
pub enum BlockSerErrorKind {
    FailedToSerializeErrorBlock(String),
    FailedToDeserializeErrorBlock(String),
}