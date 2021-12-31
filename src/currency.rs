use crate::contract::*;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WalletValue {
    pub transactions: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub date: u64,
    pub id: u32,
    pub transaction: Transaction,

    /// Hash of previous entry history_hash + previous fields in this struct 
    /// (date, id, and transaction)
    pub history_hash : Hash,
}

#[derive(Serialize, Deserialize)]
pub enum Transaction {
    Transfer {
        sender: PublicKey,
        receiver: PublicKey,
        amount: Decimal,
        /// Must be signed by the sender
        signature : Signature,
    },
    Witness {
        witnessed : PublicKey,
        by : PublicKey,
    },
}

