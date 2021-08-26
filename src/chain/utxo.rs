use crate::chain::chain::BlockChain;
use crate::tx::transaction::Transaction;
use crate::tx::tx_output::TxOutput;
use std::collections::HashMap;

impl BlockChain {
    pub fn find_unspent_transactions(&self, address: String) -> Vec<Transaction> {
        let mut unspent_transactions = vec![];
        let mut spent_txos = HashMap::<String, Vec<i32>>::new();
        let mut iter = self.into_iter();
        loop {
            if let Some(block) = iter.next() {
                block.transactions.iter().for_each(|t| {
                    let tx_id = hex::ToHex::encode_hex::<String>(&t.id);
                    'outputs: for (out_idx, out) in t.outputs.iter().enumerate() {
                        if let Some(v) = spent_txos.get(&tx_id) {
                            for spent_out in v {
                                if *spent_out == out_idx as i32 {
                                    continue 'outputs;
                                }
                            }
                        }
                        if out.can_be_unlocked(&address) {
                            unspent_transactions.push(t.clone())
                        }
                    }
                    if !t.is_coinbase() {
                        t.inputs.iter().for_each(|i| {
                            if i.can_unlock(&address) {
                                let in_tx_id = hex::ToHex::encode_hex::<String>(&i.id);
                                if let Some(v) = spent_txos.get_mut(&in_tx_id) {
                                    v.push(i.out);
                                } else {
                                    spent_txos.insert(in_tx_id, vec![i.out]);
                                }
                            }
                        })
                    }
                });
                if block.prev_hash.is_empty() {
                    break;
                }
            };
        }
        unspent_transactions
    }

    pub fn find_utxo(&self, address: String) -> Vec<TxOutput> {
        let mut utxos = vec![];
        let unspent_txns = self.find_unspent_transactions(address.clone());
        unspent_txns.iter().for_each(|tx| {
            tx.outputs.iter().for_each(|out| {
                if out.can_be_unlocked(&address) {
                    utxos.push(out.clone())
                }
            })
        });
        utxos
    }

    pub fn find_spendable_outputs(
        &self,
        address: String,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outs = HashMap::<String, Vec<i32>>::new();
        let unspent_txs = self.find_unspent_transactions(address.clone());
        let mut accumulated = 0;
        'work: for tx in &unspent_txs {
            let tx_id = hex::ToHex::encode_hex::<String>(&tx.id);
            for (out_idx, out) in tx.outputs.iter().enumerate() {
                if out.can_be_unlocked(&address) && accumulated < amount {
                    accumulated += out.value;
                    if let Some(v) = unspent_outs.get_mut(&tx_id) {
                        v.push(out_idx as i32);
                    } else {
                        unspent_outs.insert(tx_id.clone(), vec![out_idx as i32]);
                    }
                    if accumulated >= amount {
                        break 'work;
                    }
                }
            }
        }
        (accumulated, unspent_outs)
    }
}
