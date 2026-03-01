use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum ClassicTransactionReceivedType {
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "sent")]
    Sent,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct ClassicTransactionReceived {
    pub receiver: String,
    pub emitter: String,
    pub description: String,
    pub amount: String,
    pub date: String,
    pub inorout: ClassicTransactionReceivedType,
}
