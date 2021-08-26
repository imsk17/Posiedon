use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxInput {
    pub id: Vec<u8>,
    pub out: i32,
    pub sig: String,
}

impl TxInput {
    pub fn can_unlock(&self, data: &String) -> bool {
        self.sig == *data
    }
}