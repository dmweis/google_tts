use std::env;
use std::fs::{create_dir, File};
use std::io::prelude::*;

fn write_to_file(name: &str, data: &[u8]) {
    let mut file = File::create(name).unwrap();
    file.write_all(data).unwrap();
    file.flush().unwrap();
}

#[tokio::main]
async fn main() {
    let google_api_key = env::var("GOOGLE_API_KEY").expect("Please set GOOGLE_API_KEY");
    let client = google_tts::GoogleTtsClient::new(google_api_key);
    let res = client
        .list_voices_with_language_code("en-US".to_owned())
        .await
        .unwrap();
    let _ = create_dir("samples");
    for voice in res.voices {
        let voice_porps = voice.try_convert_to_voice_props().unwrap();
        let res = client
            .synthesize(
                google_tts::TextInput::with_text("Test string".to_owned()),
                voice_porps,
                google_tts::AudioConfig::default_with_encoding(google_tts::AudioEncoding::Mp3),
            )
            .await
            .unwrap();
        let path = format!("samples/{}.mp3", voice.name);
        println!("{} bytes written to {}", res.as_base_64().len(), path);
        write_to_file(&path, &res.as_byte_stream().unwrap());
    }
}
