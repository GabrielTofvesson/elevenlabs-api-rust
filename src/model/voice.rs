use std::{collections::HashMap, path::Path};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct Sample {
    pub sample_id: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: u32,
    pub hash: String,
}

#[derive(Debug, Deserialize)]
pub enum FineTuningState {
    #[serde(rename = "not_started")]
    NotStarted,
    #[serde(rename = "is_fine_tuning")]
    IsFineTuning,
    #[serde(rename = "fine_tuned")]
    FineTuned,
}

#[derive(Debug, Deserialize)]
pub struct FineTuning {
    pub is_allowed_to_fine_tune: bool,
    pub fine_tuning_requesed: bool,
    pub finetuning_state: FineTuningState,
    pub verification_attempts_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoiceSettings {
    pub stability: f32,
    pub similiarity_boost: f32,
}

#[derive(Debug, Deserialize)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    pub samples: Vec<Sample>,
    pub category: String,
    pub fine_tuning: FineTuning,
    pub preview_url: String,
    pub available_for_tiers: Vec<String>,
    pub settings: VoiceSettings,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct VoiceCreation {
    pub(crate) name: String,
    pub(crate) files: Vec<(String, Vec<u8>)>,
    pub(crate) labels: HashMap<String, String>,
}

impl VoiceCreation {
    pub fn new(name: String, files: Vec<(String, Vec<u8>)>, labels: HashMap<String, String>) -> Self {
        Self { name, files, labels }
    }

    pub fn new_files(name: String, files: Vec<&Path>, labels: HashMap<String, String>) -> std::io::Result<Self> {
        let mut collect = Vec::new();
        for path in files {
            collect.push((
                path.file_name().unwrap().to_str().unwrap().to_string(),
                std::io::Read::bytes(std::fs::File::open(path)?).map_while(|it| it.ok()).collect()
            ));
        }

        Ok(Self { name, files: collect, labels })
    }
}

#[derive(Debug, Deserialize)]
pub struct VoiceId {
    pub voice_id: String,
}