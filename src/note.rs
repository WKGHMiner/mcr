use serde::{Deserialize, Serialize};

use super::Beat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Note {
    Sound {
        beat: Beat,
        sound: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        vol: Option<i32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        column: Option<u8>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        offset: Option<i32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        sound_type: Option<i32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "endbeat")]
        end_beat: Option<Beat>,
    },
    Normal {
        beat: Beat,
        #[serde(rename = "endbeat")]
        #[serde(skip_serializing_if = "Option::is_none")]
        end_beat: Option<Beat>,
        column: u8,
    },
}
