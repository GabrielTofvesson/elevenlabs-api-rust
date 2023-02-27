pub mod model;
pub mod api;
pub mod elevenlabs_api;

extern crate reqwest;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_api() -> elevenlabs_api::ElevenLabsAPI {
        elevenlabs_api::ElevenLabsAPI::new(std::fs::read_to_string(std::path::Path::new("apikey.txt")).unwrap().trim().to_string())
    }

    #[tokio::test]
    async fn get_user_info() {
        let api = get_api();

        let result = api.get_user_info().await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.xi_api_key, api.api_key);
    }

    #[tokio::test]
    async fn get_history_items() {
        let api = get_api();

        let result = api.get_history_items().await;
        assert!(result.is_ok());

        println!("{:?}", result.unwrap());
    }
}
