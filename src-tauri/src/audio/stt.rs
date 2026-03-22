use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
use hound::{WavReader, SampleFormat, Error};
use std::{path::PathBuf, io::Cursor};

pub struct SttEngine {
    context: WhisperContext,
}

impl SttEngine {
    pub fn new(path: PathBuf) -> Result<SttEngine, String> {
        let params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(path, params).map_err(|e| e.to_string())?;
        Ok(SttEngine {
            context
        })
    }

    pub fn transcribe_wav(&self, wav_bytes: &[u8]) -> Result<String, String> {
        let cursor = Cursor::new(wav_bytes);
        let mut reader = WavReader::new(cursor).unwrap();

        let spec = reader.spec();
        if spec.bits_per_sample != 16 || spec.sample_format != SampleFormat::Int {
            return Err(format!("{}", Error::Unsupported))
        }

        let mut audio_samples = Vec::new();

        for sample in reader.samples::<i16>() {
            let sample_i16 = sample.unwrap();
            let sample_f32 = sample_i16 as f32 / 32768.0;
            audio_samples.push(sample_f32);
        }

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        params.set_language(Some("en"));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_print_timestamps(false);
        params.set_n_threads(1);

        let mut state = self.context.create_state().unwrap();
        state.full(params, &audio_samples).unwrap();

        let mut result_text = String::new();
        let n_segments = state.full_n_segments();

        for i in 0..n_segments {
            if let Some(segment) = state.get_segment(i) {
                let segment_text = segment.to_str().unwrap();
                if !segment_text.is_empty() {
                    if !result_text.is_empty() {
                        result_text.push(' ');
                    }
                    result_text.push_str(segment_text);
                }
            }
        }

        Ok(result_text)
    }
}