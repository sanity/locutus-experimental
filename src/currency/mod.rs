use crate::{
    contract::*,
    util::{PublicKey, Signature, WithBytes},
};
use rust_decimal::Decimal;

mod convert;

struct CurrencyState {
    pub transactions: Vec<(WithBytes<(Transaction, Hash)>, Signature)>,
}

struct Transaction {
    sender: PublicKey,
    receiver: PublicKey,
    amount: Decimal,
    balance: Decimal,
}

impl From<Vec<u8>> for Transaction {
    fn from(bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl From<Transaction> for Vec<u8> {
    fn from(transaction: Transaction) -> Self {
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
        for transaction in state.transactions.iter() {
            let transaction_hash = Hash::new(&transaction.0.bytes);
            running_hash.add(&transaction_hash.as_bytes());
            if parameters
                .public_key
                .verify_signature(&transaction.1, &running_hash.as_bytes()) {
                return false;
            }
            // Should use structs instead of tuples
            running_balance += transaction.0.value.0.amount;
            if running_balance != transaction.0.value.0.balance {
                return false;
            }
        }
        todo!()
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
