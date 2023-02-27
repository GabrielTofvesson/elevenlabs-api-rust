use crate::model::user::User;
use crate::model::user::subscription::Subscription;
use crate::model::error::{HTTPValidationError, APIError};
use crate::elevenlabs_api::{ElevenLabsAPI, self};

impl ElevenLabsAPI {
    pub async fn get_subscription(&self) -> Result<Subscription, APIError> {
        let response = self.get(elevenlabs_api::user::GET::Subscription)?.send().await?;
    
        if response.status().is_success() {
            let subscription: Subscription = response.json().await?;
            Ok(subscription)
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }
    
    pub async fn get_user_info(&self) -> Result<User, APIError> {
        let response = self.get(elevenlabs_api::user::GET::User)?.send().await?;
    
        if response.status().is_success() {
            let user: User = response.json().await?;
            Ok(user)
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }
}