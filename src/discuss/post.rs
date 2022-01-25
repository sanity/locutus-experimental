use crate::util::*;

pub struct PostContract {}

impl PostContract {
    pub fn validate(&self) -> bool {
        todo!();
    }
}

pub struct PostContractValue {
    pub post: Post,
    pub signature: Signature,
}

pub struct Uid(u64);

pub struct Post {
    uid: Uid,
    parent: Option<Uid>,
    author: PublicKey,
    date: u64,
    body: String,
    signature: Signature,
    stake: Uid, // TODO: figure out
}
