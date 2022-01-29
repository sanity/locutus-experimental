use std::time::Instant;

use crate::{
    contract::*,
    util::*,
};
use rust_decimal::Decimal;

mod convert;

struct CurrencyState {
    pub entries: Vec<Signed<WithRunningHash<Entry>>>,
}

enum Entry {
    Transaction {
        sender: PublicKey,
        receiver: PublicKey,
        amount: Decimal,
        balance: Decimal,
    },
    Witness {
        hash_time: (Hash, Instant),
        signature: Signature,
        public_key: PublicKey,
        predigree: Vec<(PublicKey, Signature)>,
    },
}

impl From<Vec<u8>> for Entry {
    fn from(bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl From<Entry> for Vec<u8> {
    fn from(entry: Entry) -> Self {
        unimplemented!()
    }
}

pub struct CurrencyMessage {}

pub struct CurrencyParameters {
    pub public_key: PublicKey,
}

// Should this be an implementation of ContactInterface on the State struct?
impl ContractInterface for CurrencyState {
    fn validate_state(parameters: &Parameters, state: &State) -> Result<(), String> {
        let state: CurrencyState = state.into();
        let parameters: CurrencyParameters = parameters.into();

        // A hash incorporating all transactions so far to lock-in history
        let mut running_hash = Hash::new(&Vec::new());
        let mut running_balance = Decimal::new(0, 0);
        for (ix, entry) in state.entries.iter().enumerate() {
            let entry_index = entry.signed_value.value.index;
            if entry_index != ix as u64 {
                return Result::Err(format!("Invalid index, expected {ix}, got {entry_index}"));
            }

            let entry_hash = Hash::new(&entry.signed_value.bytes);
            running_hash.add(&entry_hash.as_bytes());
            if !parameters
                .public_key
                .verify_signature(&entry.signature, &running_hash.as_bytes())
            {
                return Result::Err(format!("Signature check failed on entry {ix}"));
            }
            // Should use structs instead of tuples?
            match &entry.signed_value.value.contents {
                Entry::Transaction {
                    sender,
                    receiver,
                    amount,
                    balance,
                } => {
                    running_balance += amount;
                    if &running_balance != balance {
                        return Result::Err(format!(
                            "Invalid balance, expected {running_balance}, got {balance}, on entry {ix}"
                        ));
                    }
                }
                Entry::Witness {
                    hash_time,
                    signature,
                    public_key,
                    predigree,
                } => {}
            }
        }
        Result::Ok(())
    }

    fn validate_message(parameters: &Parameters, message: &Message) -> bool {
        todo!()
    }

    fn update_state(parameters: &Parameters, state: &mut State, message: Message) -> bool {
        todo!()
    }

    fn related_contracts(
        parameters: &Parameters,
        message: &Message,
    ) -> Vec<(Contract, Parameters)> {
        todo!()
    }

    fn summarize_state(parameters: &Parameters, state: &State) -> StateSummary {
        todo!()
    }

    fn get_state_delta(
        parameters: &Parameters,
        state: &State,
        delta_to: StateSummary,
    ) -> StateDelta {
        todo!()
    }

    fn apply_state_delta(parameters: &Parameters, state: &mut State, delta: &StateDelta) -> bool {
        todo!()
    }

    fn request(parameters: &Parameters, query: &Query) -> Response {
        todo!()
    }
}

/*
A-B-C-D

A-B-C-E
*/
