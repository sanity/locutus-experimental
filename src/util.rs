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
