use crate::contract::*;
use rust_decimal::Decimal;

pub struct WalletContract {}

pub struct WalletValue {
    pub transactions : Vec<(Vec<u8>, RSASignature)>,
}

pub struct Transaction {
    pub date : u64,
    pub from : RSAPublicKey,
    pub to : RSAPublicKey,
    pub amount : Decimal,
}
