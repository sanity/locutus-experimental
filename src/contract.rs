use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
pub struct Parameters(Vec<u8>);

pub struct State(Vec<u8>);

pub struct Response(Vec<u8>);

pub struct Message(Vec<u8>);

pub struct Query(Vec<u8>);

pub struct StateSummary(Vec<u8>);

pub struct StateDelta(Vec<u8>);

pub enum ContractKeyResult {}

pub struct Contract {}

pub trait ContractInterface {
    // Validation functions
    fn validate_state(parameters: &Parameters, state: &State) -> Result<(), String>;

    fn validate_message(parameters: &Parameters, message: &Message) -> bool;

    /// if the state has been changed persist the changes
    fn update_state(parameters: &Parameters, state: &mut State, message: Message) -> bool;

    /// relay this message to each of the contract + parameters tuple given back by the function
    fn related_contracts(parameters: &Parameters, message: &Message)
        -> Vec<(Contract, Parameters)>;

    // node layer:
    // These functions facilitate more efficient synchronization of state between peers
    fn summarize_state(parameters: &Parameters, state: &State) -> StateSummary;

    fn get_state_delta(
        parameters: &Parameters,
        state: &State,
        delta_to: StateSummary,
    ) -> StateDelta;

    fn apply_state_delta(parameters: &Parameters, state: &mut State, delta: &StateDelta) -> bool;

    // appliction layer:
    fn request(parameters: &Parameters, query: &Query) -> Response;
}


