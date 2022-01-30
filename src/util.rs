use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublicKey {}
impl PublicKey {
    pub fn verify_signature(&self, signature: &Signature, content: &Vec<u8>) -> bool {
        todo!();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Signature {}

#[derive(Serialize, Deserialize)]
pub struct Hash {}

impl Hash {
    pub fn new(bytes: &Vec<u8>) -> Self {
        todo!()
    }

    pub fn add(&mut self, bytes: &Vec<u8>) -> Self {
        todo!()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

pub struct SigCheckFailed {}

pub struct WithBytes<T> {
    pub value: T,
    pub bytes: Vec<u8>,
}

impl<T> WithBytes<T> {
    fn new(bytes: Vec<u8>, deserialize: fn(&Vec<u8>) -> T) -> Self {
        let value = deserialize(&bytes);
        Self { bytes, value }
    }
}
// fun RSAPublicKey.verify(signature : RSASignature, toVerify : ByteArray) : Boolean {
impl PublicKey {
    pub(crate) fn verify(&self, signature: &Signature, to_verify: &[u8]) -> bool {
        todo!()
    }
}
pub struct Signed<T> {
    pub signed_value: WithBytes<T>,
    pub signature: Signature,
}

pub struct WithRunningHash<T> {
    pub hashed_value: WithBytes<T>,
    pub running_hash: Hash,
}

pub struct Indexed<T> {
    pub index: u64,
    pub indexed_value: T,
}