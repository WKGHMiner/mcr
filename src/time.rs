use serde::{Deserialize, Serialize};

use super::Beat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmEvent {
    pub beat: Beat,
    pub bpm: f64,
}
