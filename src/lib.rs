pub mod model;
pub mod api;
pub mod elevenlabs_api;

extern crate reqwest;
extern crate zip;

#[cfg(test)]
mod tests {
    use crate::model::history::HistoryItems;

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

        let result = result.unwrap();

        let item = result.history.last().unwrap();

        let single_audio = api.get_history_audio(item.history_item_id.clone()).await;
        assert!(single_audio.is_ok());

        //std::fs::write("test0.mp3", single_audio.audio(0).unwrap()).unwrap();

        if result.history.len() > 1 {
            let audio_result = api.download_history(HistoryItems {
                history_item_ids: result.history[0..=1].iter().map(|x| x.history_item_id.clone()).collect()
            }).await;

            assert!(audio_result.is_ok());

            let audio_result = audio_result.unwrap();
            let audio = audio_result.audio();
            assert!(audio.len() == 2);

            //std::fs::write("test1.mp3", audio.audio(0).unwrap()).unwrap();
            //std::fs::write("test2.mp3", audio.audio(1).unwrap()).unwrap();
        }
    }
}
