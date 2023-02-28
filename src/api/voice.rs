use reqwest::multipart::Part;

use crate::{elevenlabs_api::ElevenLabsAPI, model::{voice::{Voice, VoiceSettings, VoiceCreation, VoiceId}, error::APIError}};



impl ElevenLabsAPI {
    pub async fn get_voices(&self) -> Result<Vec<Voice>, APIError> {
        let response = self.get(crate::elevenlabs_api::voice::GET::List)?.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn get_default_voice_settings(&self) -> Result<VoiceSettings, APIError> {
        let response = self.get(crate::elevenlabs_api::voice::GET::DefaultSettings)?.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn get_voice_settings(&self, voice_id: String) -> Result<VoiceSettings, APIError> {
        let response = self.get(crate::elevenlabs_api::voice::GET::Settings { voice_id })?.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn get_voice(&self, voice_id: String) -> Result<Voice, APIError> {
        let response = self.get(crate::elevenlabs_api::voice::GET::Voice { voice_id })?.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn delete_voice(&self, voice_id: String) -> Result<String, APIError> {
        let response = self.delete(crate::elevenlabs_api::voice::DELETE::Voice { voice_id })?.send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn edit_voice_settings(&self, voice_id: String, settings: VoiceSettings) -> Result<String, APIError> {
        let response = self.post(crate::elevenlabs_api::voice::POST::EditSettings { voice_id })?.json(&settings).send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn add_voice(&self, voice: VoiceCreation) -> Result<VoiceId, APIError> {
        let mut form = reqwest::multipart::Form::new().text("name", voice.name);
        for (name, file) in voice.files {
            form = form.part("files", Part::bytes(file).file_name(name));
        }

        let response = self.post(crate::elevenlabs_api::voice::POST::AddVoice)?.multipart(form).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn edit_voice(&self, voice_id: String, voice: VoiceCreation) -> Result<String, APIError> {
        let mut form = reqwest::multipart::Form::new().text("name", voice.name);
        for (name, file) in voice.files {
            form = form.part("files", Part::bytes(file).file_name(name));
        }

        let response = self.post(crate::elevenlabs_api::voice::POST::EditVoice { voice_id })?.multipart(form).send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn add_professional_voice(&self, voice: VoiceCreation) -> Result<VoiceId, APIError> {
        let mut form = reqwest::multipart::Form::new().text("name", voice.name);
        for (name, file) in voice.files {
            form = form.part("files", Part::bytes(file).file_name(name));
        }

        let response = self.post(crate::elevenlabs_api::voice::POST::AddProfessionalVoice)?.multipart(form).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }

    pub async fn start_fine_tuning_voice(&self, voice_id: String) -> Result<String, APIError> {
        let response = self.post(crate::elevenlabs_api::voice::POST::StartFineTuning { voice_id })?.send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            let error: crate::model::error::HTTPValidationError = response.json().await?;
            Err(crate::model::error::APIError::HTTPError(error))
        }
    }
}