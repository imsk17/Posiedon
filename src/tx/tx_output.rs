use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxOutput {
    pub value: i32,
    pub pub_key: String,
}

impl TxOutput {
    pub fn can_be_unlocked(&self, data: &String) -> bool {
        self.pub_key == *data
    }
}
