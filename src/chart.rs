use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{Effect, MetaData, Note, BpmEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub meta: MetaData,
    pub time: Vec<BpmEvent>,
    pub effect: Vec<Effect>,
    pub note: Vec<Note>,
}

impl FromStr for Chart {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl From<String> for Chart {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap()
    }
}
