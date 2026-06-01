use serde::{Deserialize, Serialize};

use super::Beat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub beat: Beat,
    pub scroll: f64,
}
