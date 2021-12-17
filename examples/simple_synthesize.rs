use std::env;

#[tokio::main]
async fn main() {
    let google_api_key = env::var("GOOGLE_API_KEY").expect("Please set GOOGLE_API_KEY");
    let client = google_tts::GoogleTtsClient::new(google_api_key);
    let res = client
        .synthesize(
            google_tts::TextInput::with_text("Test string".to_owned()),
            google_tts::VoiceProps::default_english_male(),
            google_tts::AudioConfig::default_with_encoding(google_tts::AudioEncoding::Mp3),
        )
        .await
        .unwrap();
    println!("Response length: {} bytes", res.as_base_64().len());
}
