use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Intention {
    pub exchange_id: String,
    pub action: String,
    pub action_params: String,
    pub pool_address: String,
    pub nonce: u64,
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IntentionSet {
    pub initiator_address: String,
    pub tx_fee_in_sats: u64,
    pub intentions: Vec<Intention>,
}
