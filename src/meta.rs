use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::SongInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "$ver")]
    pub ver: i32,
    pub creator: String,
    pub background: String,
    pub version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<u64>,
    pub id: u64,
    pub mode: u32,
    pub time: u64,
    pub song: SongInfo,
    pub mode_ext: HashMap<String, Value>,
}
