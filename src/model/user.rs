use serde::Deserialize;

pub mod subscription {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Subscription {
        pub tier: String,
        pub character_count: u32,
        pub character_limit: u32,
        pub can_extend_character_limit: bool,
        pub allowed_to_extend_character_limit: bool,
        pub next_character_count_reset_unix: u64,
        pub voice_limit: u32,
        pub can_extend_voice_limit: bool,
        pub can_use_instant_voice_cloning: bool,
        pub available_models: Vec<TTSModel>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TTSModel {
        pub model_id: String,
        pub display_name: String,
        pub supported_languages: Vec<Language>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Language {
        pub iso_code: String,
        pub display_name: String,
    }

    #[derive(Debug, Deserialize)]
    pub enum Status {
        #[serde(rename = "trialing")]
        Trialing,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "past_due")]
        Incomplete,
        #[serde(rename = "incomplete")]
        IncompleteExpired,
        #[serde(rename = "incomplete_expired")]
        PastDue,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "unpaid")]
        Unpaid,
        #[serde(rename = "free")]
        Free,
    }

    #[derive(Debug, Deserialize)]
    pub struct Invoice {
        pub amount_due_cents: u32,
        pub next_payment_attempt_unix: u64,
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub subscription: subscription::Subscription,
    pub is_new_user: bool,
    pub xi_api_key: String,
}