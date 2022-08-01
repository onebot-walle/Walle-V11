use serde::{Deserialize, Serialize};
use walle_core::util::ValueMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub time: u64,
    pub self_id: i64,
    pub post_type: String,
    #[serde(flatten)]
    pub content: ValueMap,
}
