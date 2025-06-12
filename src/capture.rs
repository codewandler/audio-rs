use crate::channel::new_audio_channel;
use crate::{AudioSource, AudioSourceFanOut, IntoAudioSource};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleRate, StreamConfig};
use crossbeam_channel::{Receiver, Sender};
use tracing::error;

pub struct AudioCapture {
    input_stream: cpal::Stream,
    tx: Sender<f32>,
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

pub fn audio_capture(sample_rate: u32) -> anyhow::Result<AudioSourceFanOut<f32, AudioCapture>> {
    let (tx, rx) = new_audio_channel::<f32>();

    // ---- Capture setup ----
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");

    // Build a StreamConfig for mono at desired sample rate
    let config = StreamConfig {
        channels: 1,
        sample_rate: SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };

    // Error callback
    let err_fn = |err| eprintln!("Input stream error: {}", err);

    let tx_cb = tx.clone();
    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for frame in data.chunks(1) {
                for &sample in frame {
                    match tx_cb.send(sample) {
                        Ok(_) => {}
                        Err(err) => {
                            error!("send failed: {:?}", err.to_string());
                        }
                    }
                }
            }
        },
        err_fn,
        None,
    )?;

    input_stream.play()?;

    let cap = AudioCapture {
        input_stream,
        tx,
        rx: Some(rx),
    };

    Ok(AudioSourceFanOut::new(cap))
}
