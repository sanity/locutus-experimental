use serde_derive::{Deserialize, Serialize};

/// The Value associated with this contract
pub type Value = Vec<u8>;

/// A payload associated with this contract, considered part of the contract
/// when determining the contract's location
pub type Payload = Vec<u8>;

#[derive(Serialize, Deserialize)]
pub struct PublicKey {}

impl PublicKey {
    pub fn verify_signature(&self, signature: Signature, content: Vec<u8>) -> bool {
        todo!();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Signature {}

#[derive(Serialize, Deserialize)]
pub struct Hash {}

pub trait ContractKey {
    /// Determine whether this value is valid for this contract
    fn validate_value(payload: &Payload, value: &Value) -> bool;

    /// Merges two values, both of which will already have been validated.
    ///
    /// The contract implementation must guarantee that merges have the commutative property
    ///     merge(a, b) == merge(b, a) and
    ///     merge(merge(a, b), c) == merge(a, merge(b, c)) == merge(merge(a, c), b)
    /// The implementation must also be able to detect whether a value has already been merged, ie:
    ///     merge(merge(a, b), b) == None
    /// Will return Some(the merged value) or None
    fn merge(payload: &Payload, existing_value: &Value, new_value: &Value) -> Option<Value>;

    // TODO: Should existing_value be mutated in-place rather than creating a new Value?
}
