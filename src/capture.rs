use crate::channel::new_audio_channel;
use crate::{AudioSource, AudioSourceFanOut, IntoAudioSource};
use anyhow::anyhow;
use crossbeam_channel::{Receiver, Sender};
use rodio::cpal::StreamConfig;
use rodio::cpal::traits::{HostTrait, StreamTrait};
use rodio::{DeviceTrait, cpal};
use rubato::{FftFixedInOut, Resampler};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tracing::debug;

pub struct AudioCapture {
    _input_stream: cpal::Stream,
    _tx: Sender<f32>,
    rx: Option<Receiver<f32>>,
}

impl IntoAudioSource for AudioCapture {
    type Format = f32;
    fn into_audio_source(&mut self) -> anyhow::Result<Box<dyn AudioSource<Format = f32>>> {
        match self.rx.take() {
            Some(x) => Ok(Box::new(x)),
            None => Err(anyhow::anyhow!("Audio capture already taken")),
        }
    }
}

/// Start audio capture in mono from default input device.
pub fn audio_capture(
    output_sample_rate: u32,
) -> anyhow::Result<AudioSourceFanOut<f32, AudioCapture>> {
    // ---- Capture setup ----
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

    let config = device.default_input_config()?;

    // Only allow f32 format as required
    if config.sample_format() != cpal::SampleFormat::F32 {
        return Err(anyhow!(
            "Expected f32 input, got {:?}",
            config.sample_format()
        ));
    }

    let input_config: StreamConfig = config.clone().into();
    let input_sample_rate = input_config.sample_rate.0;
    let channels = input_config.channels;

    debug!(
        "Input device: {}, Sample rate: {}, Channels: {}, Output rate: {}",
        device.name()?,
        input_sample_rate,
        channels,
        output_sample_rate,
    );

    // Resampler
    let resampler = Arc::new(Mutex::new(FftFixedInOut::<f32>::new(
        input_sample_rate as usize,
        output_sample_rate as usize,
        1024,
        1,
    )?));

    // resample config
    let buf_size = resampler.lock().unwrap().input_frames_next();
    let sample_buffer = Arc::new(Mutex::new(VecDeque::<f32>::new()));
    let resampler_clone = resampler.clone();
    let buffer_clone = sample_buffer.clone();

    // TODO: use a ring buffer instead of a vector

    let (tx, rx) = new_audio_channel::<f32>();
    let tx_cb = tx.clone();
    let input_stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Downmix stereo to mono if needed
            let mono = if channels == 1 {
                data.to_vec()
            } else if channels == 2 {
                stereo_to_mono(data)
            } else {
                eprintln!("Unsupported channel count: {}", channels);
                return;
            };

            let mut buf = buffer_clone.lock().unwrap();
            buf.extend(mono);

            while buf.len() >= buf_size {
                let input_block: Vec<f32> = buf.drain(..buf_size).collect();
                let mut resampler = resampler_clone.lock().unwrap();
                match resampler.process(&[input_block], None) {
                    Ok(output) => {
                        let resampled = &output[0];
                        for sample in resampled {
                            tx_cb.send(*sample).unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!("Resample error: {:?}", e);
                    }
                }
            }
        },
        |err| eprintln!("Input stream error: {}", err),
        None,
    )?;

    input_stream.play()?;

    let cap = AudioCapture {
        _input_stream: input_stream,
        _tx: tx,
        rx: Some(rx),
    };

    Ok(AudioSourceFanOut::new(cap))
}

/// Convert stereo interleaved f32 to mono (average L + R)
fn stereo_to_mono(input: &[f32]) -> Vec<f32> {
    input
        .chunks_exact(2)
        .map(|chunk| 0.5 * (chunk[0] + chunk[1]))
        .collect()
}
