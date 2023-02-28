use serde::Serialize;

use super::voice::VoiceSettings;

#[derive(Debug, Serialize)]
pub struct TTSMessage {
    pub(crate) text: String,
    pub(crate) voice_settings: VoiceSettings,
}

impl TTSMessage {
    pub fn new(text: String, voice_settings: VoiceSettings) -> Self {
        Self {
            text,
            voice_settings,
        }
    }
}