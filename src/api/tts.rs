use crate::{elevenlabs_api::ElevenLabsAPI, model::{history::AudioItem, error::{HTTPValidationError, APIError}, tts::TTSMessage}};



impl ElevenLabsAPI {
    pub async fn generate_tts(&self, voice_id: String, message: TTSMessage) -> Result<AudioItem, APIError> {
        let response = self.post(crate::elevenlabs_api::tts::POST::File { voice_id })?.json(&message).send().await?;

        if response.status().is_success() {
            Ok(AudioItem::new_single(response.bytes().await?))
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }

    // TODO: Consider stream implementation
}