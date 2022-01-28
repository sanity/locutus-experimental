use crate::{
    contract::*,
    util::{PublicKey, Signature, WithBytes},
};
use rust_decimal::Decimal;

mod convert;

struct CurrencyState {
    pub entries: Vec<Signed<WithRunningHash<Entry>>>,
}

struct Signed<T> {
    pub signed_value : WithBytes<T>,
    pub signature : Signature,
}

struct WithRunningHash<T> {
    pub contents : T,
    pub running_hash : Hash,
}

enum Entry {
    Transaction {
        sender: PublicKey,
        receiver: PublicKey,
        amount: Decimal,
        balance: Decimal,
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
    fn validate_state(parameters: &Parameters, state: &State) -> bool {
        let state: CurrencyState = state.into();
        let parameters: CurrencyParameters = parameters.into();

        // A hash incorporating all transactions so far to lock-in history
        let mut running_hash = Hash::new(&Vec::new());
        let mut running_balance = Decimal::new(0, 0);
        for entry in state.entries.iter() {
            let entry_hash = Hash::new(&entry.signed_value.bytes);
            running_hash.add(&entry_hash.as_bytes());
            if parameters
                .public_key
                .verify_signature(&entry.signature, &running_hash.as_bytes())
            {
                return false;
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
                        return false;
                    }
                }
            }
        }
        true
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
