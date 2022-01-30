use crate::util::*;

pub struct ArbiterState {
    pub entries : Vec<Signed<WithRunningHash<Entry>>>,
}

pub struct Entry {
    
}