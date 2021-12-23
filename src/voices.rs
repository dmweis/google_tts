use crate::{SsmlVoiceGender, VoiceProps};

pub enum EnUsVoices {
    EnUsWavenetA,
    EnUsWavenetB,
    EnUsWavenetC,
    EnUsWavenetD,
    EnUsWavenetE,
    EnUsWavenetF,
    EnUsWavenetG,
    EnUsWavenetH,
    EnUsWavenetI,
    EnUsWavenetJ,
    EnUsStandardA,
    EnUsStandardB,
    EnUsStandardC,
    EnUsStandardD,
    EnUsStandardE,
    EnUsStandardF,
    EnUsStandardG,
    EnUsStandardH,
    EnUsStandardI,
    EnUsStandardJ,
}

impl EnUsVoices {
    pub fn as_voice_prop(&self) -> VoiceProps {
        match self {
            // Standard
            EnUsVoices::EnUsStandardA => {
                VoiceProps::with_all("en-US", "en-US-Standard-A", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsStandardB => {
                VoiceProps::with_all("en-US", "en-US-Standard-B", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsStandardC => {
                VoiceProps::with_all("en-US", "en-US-Standard-C", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsStandardD => {
                VoiceProps::with_all("en-US", "en-US-Standard-D", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsStandardE => {
                VoiceProps::with_all("en-US", "en-US-Standard-E", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsStandardF => {
                VoiceProps::with_all("en-US", "en-US-Standard-F", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsStandardG => {
                VoiceProps::with_all("en-US", "en-US-Standard-G", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsStandardH => {
                VoiceProps::with_all("en-US", "en-US-Standard-H", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsStandardI => {
                VoiceProps::with_all("en-US", "en-US-Standard-I", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsStandardJ => {
                VoiceProps::with_all("en-US", "en-US-Standard-J", SsmlVoiceGender::Male)
            }
            // Wavenet
            EnUsVoices::EnUsWavenetA => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-A", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsWavenetB => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-B", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsWavenetC => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-C", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsWavenetD => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-D", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsWavenetE => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-E", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsWavenetF => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-F", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsWavenetG => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-G", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsWavenetH => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-H", SsmlVoiceGender::Female)
            }
            EnUsVoices::EnUsWavenetI => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-I", SsmlVoiceGender::Male)
            }
            EnUsVoices::EnUsWavenetJ => {
                VoiceProps::with_all("en-US", "en-US-Wavenet-J", SsmlVoiceGender::Male)
            }
        }
    }
}
