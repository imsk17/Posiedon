use crate::tx::tx_input::TxInput;
use crate::tx::tx_output::TxOutput;
use sha2::Digest;
use serde::{Serialize, Deserialize};
use crate::chain::chain::BlockChain;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}


impl Transaction {
    pub fn new(from: String, to: String, amount: i32, chain: &BlockChain) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        let (acc, valid_outputs) = chain.find_spendable_outputs(from.clone(), amount);
        if acc < amount {
            panic!("not enough funds!")
        };
        for (txid, outs) in valid_outputs {
            let tx_id = hex::decode(txid).unwrap();
            for out in outs {
                let input = TxInput {
                    id: tx_id.clone(),
                    out,
                    sig: from.clone(),
                };
                inputs.push(input);
            }
        };
        outputs.push(TxOutput {
            value: amount,
            pub_key: to,
        });
        if acc > amount {
            outputs.push(TxOutput {
                pub_key: from.clone(),
                value: acc - amount,
            })
        };
        let mut tx = Self {
            id: vec![],
            outputs,
            inputs,
        };
        tx.set_id();
        tx
    }

    pub fn coinbase_tx(to: String, data: String) -> Self {
        let d = if data == "" {
            format!("Coins to {}", to)
        } else {
            data
        };
        let inputs = TxInput {
            id: vec![],
            out: -1,
            sig: d,
        };
        let outputs = TxOutput {
            pub_key: to,
            value: 100,
        };
        let mut transaction = Transaction {
            outputs: vec![outputs],
            inputs: vec![inputs],
            id: vec![],
        };
        transaction.set_id();
        transaction
    }

    pub fn set_id(&mut self) {
        let tx = bincode::serialize(&self).expect("failed to serialize transaction");
        let hash = sha2::Sha256::digest(&tx).to_vec();
        self.id = hash
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].id.len() == 0 && self.inputs[0].out == -1
    }
}