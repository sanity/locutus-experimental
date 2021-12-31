use crate::contract::*;

struct TweetContract {}

pub struct TweetValue {
    number: u32,
    content: Vec<u8>,
    signature: Signature,
}

impl TweetValue {
    pub fn new(value: &Value) -> Self {
        todo!()
    }
}

impl TryInto<Vec<u8>> for TweetValue {
    type Error = ();

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}

pub struct TweetPayload {
    public_key: PublicKey,
}

impl TweetPayload {
    pub fn new(payload: &Payload) -> Self {
        todo!()
    }
}

impl ContractKey for TweetContract {
    fn validate_value(payload: &Payload, value: &Value) -> bool {
        let value = TweetValue::new(value);
        let payload = TweetPayload::new(payload);
        payload
            .public_key
            .verify_signature(value.signature, value.content)
    }

    fn merge(payload: &Payload, existing_value: &Value, new_value: &Value) -> Option<Vec<u8>> {
        let existing_value = TweetValue::new(existing_value);
        let new_value = TweetValue::new(new_value);
        if (new_value.number > existing_value.number) {
            Some(new_value.try_into().unwrap())
        } else {
            Some(existing_value.try_into().unwrap())
        }
    }
}
