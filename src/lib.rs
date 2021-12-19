use base64::decode;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Clone, Debug)]
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

    pub fn default_english_female_wavenet() -> VoiceProps {
        VoiceProps {
            language_code: "en-US".to_owned(),
            name: Some("en-US-Wavenet-C".to_owned()),
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

#[derive(Serialize, Clone, Copy, Debug)]
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

#[derive(Serialize, Clone, Debug)]
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
    pub fn new(
        audio_encoding: AudioEncoding,
        speaking_rate: Option<f32>,
        pitch: Option<i32>,
        volume_gain_db: Option<f32>,
        sample_rate_hertz: Option<i32>,
        effects_profile_id: Option<Vec<String>>,
    ) -> Self {
        Self {
            audio_encoding,
            speaking_rate,
            pitch,
            volume_gain_db,
            sample_rate_hertz,
            effects_profile_id,
        }
    }

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

#[derive(Serialize, Clone, Debug)]
struct TtsRequest {
    input: TextInput,
    voice: VoiceProps,
    #[serde(alias = "audioConfig")]
    audio_config: AudioConfig,
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VoiceDescription {
    #[serde(alias = "languageCodes")]
    pub language_codes: Vec<String>,
    pub name: String,
    #[serde(alias = "ssmlGender")]
    pub ssml_gender: SsmlVoiceGender,
    #[serde(alias = "naturalSampleRateHertz")]
    pub natural_sample_rate_hertz: i32,
}

impl VoiceDescription {
    pub fn try_convert_to_voice_props(&self) -> Result<VoiceProps, Box<dyn Error>> {
        Ok(VoiceProps {
            // Use thiserror for this
            language_code: self
                .language_codes
                .get(0)
                .ok_or("Failed to convert Voice description to voice properties")?
                .clone(),
            name: Some(self.name.clone()),
            ssml_gender: Some(self.ssml_gender),
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ListVoicesResponse {
    pub voices: Vec<VoiceDescription>,
}

impl ListVoicesResponse {
    pub fn as_json(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

pub struct GoogleTtsClient {
    api_key: String,
    https_client: reqwest::Client,
    synthesize_endpoint_url: String,
    list_voices_endpoint_url: String,
}

impl GoogleTtsClient {
    pub fn new(api_key: String) -> GoogleTtsClient {
        let client = reqwest::Client::new();

        #[cfg(not(test))]
        let base_url = String::from("https://texttospeech.googleapis.com");
        #[cfg(test)]
        let base_url = mockito::server_url();

        // Use url join instead of format
        let synthesize_endpoint_url = format!("{}{}", base_url, "/v1/text:synthesize");
        let list_voices_endpoint_url = format!("{}{}", base_url, "/v1/voices");

        GoogleTtsClient {
            api_key,
            https_client: client,
            synthesize_endpoint_url,
            list_voices_endpoint_url,
        }
    }

    pub async fn synthesize(
        &self,
        input: TextInput,
        voice: VoiceProps,
        audio: AudioConfig,
    ) -> Result<TtsResponse, Box<dyn Error>> {
        let req = TtsRequest {
            input,
            voice,
            audio_config: audio,
        };
        let url = Url::parse_with_params(
            &self.synthesize_endpoint_url,
            &[("alt", "json"), ("key", &self.api_key)],
        )?;
        let res: TtsResponse = self
            .https_client
            .post(url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn list_voices(&self) -> Result<ListVoicesResponse, Box<dyn Error>> {
        let url =
            Url::parse_with_params(&self.list_voices_endpoint_url, &[("key", &self.api_key)])?;
        let res: ListVoicesResponse = self.https_client.get(url).send().await?.json().await?;
        Ok(res)
    }

    pub async fn list_voices_with_language_code(
        &self,
        language_code: String,
    ) -> Result<ListVoicesResponse, Box<dyn Error>> {
        let url = Url::parse_with_params(
            &self.list_voices_endpoint_url,
            &[("key", &self.api_key), ("languageCode", &language_code)],
        )?;
        let res: ListVoicesResponse = self.https_client.get(url).send().await?.json().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

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

    #[tokio::test]
    async fn simple_tts_request() {
        let mock_tts_api = mock("POST", "/v1/text:synthesize")
            .match_query(Matcher::UrlEncoded("alt".into(), "json".into()))
            .match_query(Matcher::UrlEncoded("key".into(), "fake-key".into()))
            .match_body(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"audioContent": "testtesttest"}"#)
            .create();

        let client = GoogleTtsClient::new("fake-key".to_owned());
        let fake_res = client
            .synthesize(
                TextInput::with_text("hi".to_owned()),
                VoiceProps::default_english_female(),
                AudioConfig::default_with_encoding(AudioEncoding::Mp3),
            )
            .await
            .unwrap();

        mock_tts_api.assert();
        assert_eq!(fake_res.as_base_64(), "testtesttest".to_owned());
    }

    #[tokio::test]
    async fn list_voices_request() {
        let mock_tts_api = mock("GET", "/v1/voices")
            .match_query(Matcher::UrlEncoded("key".into(), "fake-key".into()))
            .match_body(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "voices": [
                    {
                        "language_codes": [
                            "ro-RO"
                        ],
                        "name": "ro-RO-Wavenet-A",
                        "ssml_gender": "FEMALE",
                        "natural_sample_rate_hertz": 24000
                    }
                    ]
                }"#,
            )
            .create();

        let client = GoogleTtsClient::new("fake-key".to_owned());
        let fake_res = client.list_voices().await.unwrap();

        mock_tts_api.assert();
        assert_eq!(fake_res.voices.len(), 1);
        assert_eq!(fake_res.voices[0].language_codes.len(), 1);
        assert_eq!(fake_res.voices[0].language_codes[0], "ro-RO");
        assert_eq!(fake_res.voices[0].name, "ro-RO-Wavenet-A");
        assert_eq!(fake_res.voices[0].ssml_gender, SsmlVoiceGender::Female);
        assert_eq!(fake_res.voices[0].natural_sample_rate_hertz, 24000);
    }

    #[tokio::test]
    async fn list_voices_request_with_lang_code() {
        let mock_tts_api = mock("GET", "/v1/voices")
            .match_query(Matcher::UrlEncoded("key".into(), "fake-key".into()))
            .match_query(Matcher::UrlEncoded("languageCode".into(), "en-US".into()))
            .match_body(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "voices": [
                    {
                        "language_codes": [
                            "en-US"
                        ],
                        "name": "ro-RO-Wavenet-A",
                        "ssml_gender": "FEMALE",
                        "natural_sample_rate_hertz": 24000
                    }
                    ]
                }"#,
            )
            .create();

        let client = GoogleTtsClient::new("fake-key".to_owned());
        let fake_res = client
            .list_voices_with_language_code("en-US".to_owned())
            .await
            .unwrap();

        mock_tts_api.assert();
        assert_eq!(fake_res.voices.len(), 1);
        assert_eq!(fake_res.voices[0].language_codes.len(), 1);
        assert_eq!(fake_res.voices[0].language_codes[0], "en-US");
        assert_eq!(fake_res.voices[0].name, "ro-RO-Wavenet-A");
        assert_eq!(fake_res.voices[0].ssml_gender, SsmlVoiceGender::Female);
        assert_eq!(fake_res.voices[0].natural_sample_rate_hertz, 24000);
    }
}
