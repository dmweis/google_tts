use base64::decode;
use reqwest::blocking;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;
#[cfg(test)]
use mockito;

#[derive(Serialize)]
pub struct TextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<String>,
}

impl TextInput {
    pub fn with_text(text: String) -> TextInput {
        TextInput {
            text: Some(text),
            ssml: None,
        }
    }

    pub fn with_ssml(ssml: String) -> TextInput {
        TextInput {
            text: None,
            ssml: Some(ssml),
        }
    }
}

#[derive(Serialize)]
pub enum SsmlVoiceGender {
    #[serde(rename = "SSML_VOICE_GENDER_UNSPECIFIED")]
    SsmlVoiceGenderUnspecified,
    #[serde(rename = "MALE")]
    Male,
    #[serde(rename = "FEMALE")]
    Female,
    #[serde(rename = "NEUTRAL")]
    Neutral,
}

#[derive(Serialize)]
pub struct VoiceProps {
    #[serde(alias = "languageCode")]
    language_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(alias = "ssmlGender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml_gender: Option<SsmlVoiceGender>,
}

impl VoiceProps {
    pub fn new(
        language_code: String,
        name: Option<String>,
        ssml_gender: Option<SsmlVoiceGender>,
    ) -> VoiceProps {
        VoiceProps {
            language_code,
            name,
            ssml_gender,
        }
    }

    pub fn default_english_female() -> VoiceProps {
        VoiceProps {
            language_code: "en-US".to_owned(),
            name: None,
            ssml_gender: Some(SsmlVoiceGender::Female),
        }
    }

    pub fn default_english_male() -> VoiceProps {
        VoiceProps {
            language_code: "en-US".to_owned(),
            name: None,
            ssml_gender: Some(SsmlVoiceGender::Male),
        }
    }
}

#[derive(Serialize)]
pub enum AudioEncoding {
    #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
    AudioEncodingUnspecified,
    #[serde(rename = "LINEAR16")]
    Linear16,
    #[serde(rename = "MP3")]
    Mp3,
    #[serde(rename = "OGG_OPUS")]
    OggOpus,
}

#[derive(Serialize)]
pub struct AudioConfig {
    #[serde(alias = "audioEncoding")]
    audio_encoding: AudioEncoding,

    #[serde(alias = "speakingRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    speaking_rate: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<i32>,

    #[serde(alias = "volumeGainDb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_gain_db: Option<f32>,

    #[serde(alias = "sampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate_hertz: Option<i32>,

    #[serde(alias = "effectsProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    effects_profile_id: Option<Vec<String>>,
}

impl AudioConfig {
    pub fn default_with_encoding(encoding: AudioEncoding) -> AudioConfig {
        AudioConfig {
            audio_encoding: encoding,
            speaking_rate: None,
            pitch: None,
            volume_gain_db: None,
            sample_rate_hertz: None,
            effects_profile_id: None,
        }
    }
}

#[derive(Serialize)]
struct TtsRequest {
    input: TextInput,
    voice: VoiceProps,
    #[serde(alias = "audioConfig")]
    audio_config: AudioConfig,
}

#[derive(Deserialize)]
pub struct TtsResponse {
    #[serde(alias = "audioContent")]
    audio_content: String,
}

impl TtsResponse {
    pub fn as_byte_stream(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let payload = decode(&self.audio_content)?;
        Ok(payload)
    }

    pub fn as_base_64(&self) -> String {
        self.audio_content.to_owned()
    }
}

pub struct GoogleTtsClient {
    api_key: String,
    https_client: blocking::Client,
    url: String,
}

impl GoogleTtsClient {
    pub fn new(api_key: String) -> GoogleTtsClient {
        let client = blocking::Client::new();
        
        #[cfg(not(test))]
        let url = "https://texttospeech.googleapis.com/v1beta1/text:synthesize";
        #[cfg(test)]
        let url = format!("{}{}", &mockito::server_url(), "/v1beta1/text:synthesize");

        GoogleTtsClient {
            api_key,
            https_client: client,
            url: url.to_owned(),
        }
    }

    pub fn synthesize(
        &self,
        input: TextInput,
        voice: VoiceProps,
        audio: AudioConfig,
    ) -> Result<TtsResponse, Box<dyn Error>> {
        let req = TtsRequest {
            input: input,
            voice: voice,
            audio_config: audio,
        };
        let url = Url::parse_with_params(
            &self.url,
            &[("alt", "json"), ("key", &self.api_key)],
        )?;
        let res: TtsResponse = self.https_client.post(url).json(&req).send()?.json()?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{ mock, Matcher };

    #[test]
    fn audio_config_serialize_none_fields() {
        let default = AudioConfig::default_with_encoding(AudioEncoding::Mp3);
        let json = serde_json::to_string(&default).unwrap();
        assert_eq!(json, "{\"audio_encoding\":\"MP3\"}");
    }

    #[test]
    fn voice_props_serialize_none_fields() {
        let default = VoiceProps::default_english_female();
        let json = serde_json::to_string(&default).unwrap();
        assert_eq!(
            json,
            "{\"language_code\":\"en-US\",\"ssml_gender\":\"FEMALE\"}"
        );
    }

    #[test]
    fn test_something() {
        let mock_tts_api = mock("POST", "/v1beta1/text:synthesize")
        .match_query(Matcher::UrlEncoded("alt".into(), "json".into()))
        .match_query(Matcher::UrlEncoded("key".into(), "fake-key".into()))
        .match_body(Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"audioContent": "testtesttest"}"#)
        .create();

        let client = GoogleTtsClient::new("fake-key".to_owned());
        let fake_res = client.synthesize(
            TextInput::with_text("hi".to_owned()),
            VoiceProps::default_english_female(),
            AudioConfig::default_with_encoding(AudioEncoding::Mp3)
        ).unwrap();
        
        mock_tts_api.assert();
        assert_eq!(fake_res.as_base_64(), "testtesttest".to_owned());
    }
}
