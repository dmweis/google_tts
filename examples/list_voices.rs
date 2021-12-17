use std::env;

#[tokio::main]
async fn main() {
    let google_api_key = env::var("GOOGLE_API_KEY").expect("Please set GOOGLE_API_KEY");
    let client = google_tts::GoogleTtsClient::new(google_api_key);
    let res = client.list_voices().await.unwrap();
    println!("{}", res.as_json().unwrap());
}
