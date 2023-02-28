use reqwest::{Error, Client, RequestBuilder};

#[derive(Debug)]
pub struct ElevenLabsAPI {
    pub host: &'static str,
    pub api_key: String,
}

impl ElevenLabsAPI {
    pub fn new(api_key: String) -> Self {
        Self {
            host: "https://api.elevenlabs.io",
            api_key,
        }
    }

    pub fn endpoint(&self, endpoint: impl Endpoint) -> String {
        format!("{}{}", self.host, endpoint.path())
    }

    fn with_header(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.header("xi-api-key", self.api_key.clone())
    }

    pub fn get(&self, endpoint: impl Endpoint) -> Result<RequestBuilder, Error> {
        Ok(self.with_header(Client::builder().build()?.get(self.endpoint(endpoint))))
    }

    pub fn post(&self, endpoint: impl Endpoint) -> Result<RequestBuilder, Error> {
        Ok(self.with_header(Client::builder().build()?.post(self.endpoint(endpoint))))
    }

    pub fn delete(&self, endpoint: impl Endpoint) -> Result<RequestBuilder, Error> {
        Ok(self.with_header(Client::builder().build()?.delete(self.endpoint(endpoint))))
    }
}

pub trait Endpoint {
    fn path(&self) -> String;
}

pub mod tts {
    use super::Endpoint;

    pub enum POST {
        File {
            voice_id: String,
        },
        Stream {
            voice_id: String,
        },
    }

    impl Endpoint for POST {
        fn path(&self) -> String {
            match self {
                POST::File { voice_id } => format!("/v1/text-to-speech/{}", voice_id),
                POST::Stream { voice_id } => format!("/v1/text-to-speech/{}/stream", voice_id),
            }
        }
    }
}

pub mod voice {
    use super::Endpoint;

    pub enum GET {
        List,
        DefaultSettings,
        Settings {
            voice_id: String,
        },
        Voice {
            voice_id: String,
        },
    }

    impl Endpoint for GET {
        fn path(&self) -> String {
            match self {
                GET::List => "/v1/voices".to_string(),
                GET::DefaultSettings => "/v1/voices/settings/default".to_string(),
                GET::Settings { voice_id } => format!("/v1/voices/{}/settings", voice_id),
                GET::Voice { voice_id } => format!("/v1/voices/{}", voice_id),
            }
        }
    }

    pub enum DELETE {
        Voice {
            voice_id: String,
        },
    }

    impl Endpoint for DELETE {
        fn path(&self) -> String {
            match self {
                DELETE::Voice { voice_id } => format!("/v1/voices/{}", voice_id),
            }
        }
    }

    pub enum POST {
        EditSettings {
            voice_id: String,
        },
        AddVoice,
        EditVoice {
            voice_id: String,
        },
        AddProfessionalVoice,
        StartFineTuning {
            voice_id: String,
        },
    }

    impl Endpoint for POST {
        fn path(&self) -> String {
            match self {
                POST::EditSettings { voice_id } => format!("/v1/voices/{}/settings/edit", voice_id),
                POST::AddVoice => "/v1/voices".to_string(),
                POST::EditVoice { voice_id } => format!("/v1/voices/{}", voice_id),
                POST::AddProfessionalVoice => "/v1/voices/add-professional".to_string(),
                POST::StartFineTuning { voice_id } => format!("/v1/voices/{}/start-fine-tuning", voice_id),
            }
        }
    }
}

pub mod samples {
    use super::Endpoint;

    pub enum GET {
        Sample {
            voice_id: String,
            sample_id: String,
        },
    }

    impl Endpoint for GET {
        fn path(&self) -> String {
            match self {
                GET::Sample { voice_id, sample_id } => format!("/v1/voices/{voice_id}/samples/{sample_id}/audio"),
            }
        }
    }

    pub enum DELETE {
        Sample {
            voice_id: String,
            sample_id: String,
        },
    }

    impl Endpoint for DELETE {
        fn path(&self) -> String {
            match self {
                DELETE::Sample { voice_id, sample_id } => format!("/v1/voices/{voice_id}/samples/{sample_id}"),
            }
        }
    }
}

pub mod history {
    use super::Endpoint;

    pub enum GET {
        History,
        HistoryItem {
            item_id: String,
        },
    }

    impl Endpoint for GET {
        fn path(&self) -> String {
            match self {
                GET::History => "/v1/history".to_string(),
                GET::HistoryItem { item_id } => format!("/v1/history/{item_id}/audio"),
            }
        }
    }
    
    pub enum DELETE {
        HistoryItem {
            item_id: String,
        },
    }

    impl Endpoint for DELETE {
        fn path(&self) -> String {
            match self {
                DELETE::HistoryItem { item_id } => format!("/v1/history/{item_id}"),
            }
        }
    }

    pub enum POST {
        DownloadHistory
    }

    impl Endpoint for POST {
        fn path(&self) -> String {
            match self {
                POST::DownloadHistory => "/v1/history/download".to_string(),
            }
        }
    }
}

pub mod user {
    use super::Endpoint;

    pub enum GET {
        Subscription,
        User,
    }

    impl Endpoint for GET {
        fn path(&self) -> String {
            match self {
                GET::Subscription => "/v1/user/subscription".to_string(),
                GET::User => "/v1/user".to_string(),
            }
        }
    }
}