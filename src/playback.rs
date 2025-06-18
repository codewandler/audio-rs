use crate::channel::new_audio_channel;
use crate::config::{StreamConfigQuery, supported_output_config};
use crate::format::SampleFormat;
use cpal::traits::HostTrait;
use crossbeam_channel::{Receiver, Sender};
use rodio::dynamic_mixer::DynamicMixerController;
use rodio::dynamic_mixer::mixer;
use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source, cpal};
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

pub struct AudioPlayback<F>
where
    F: SampleFormat,
{
    _stream: OutputStream,
    _sink: Arc<Sink>,
    _mixer_handle: Arc<DynamicMixerController<F>>,
}

pub struct AudioPlaybackOutput<F>
where
    F: SampleFormat,
{
    sample_rate: u32,
    _tx: Sender<F>,
    rx: Receiver<F>,
}

impl Source for AudioPlaybackOutput<f32> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for AudioPlaybackOutput<f32> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rx.try_recv() {
            Ok(data) => Some(data),
            Err(err) => match err {
                crossbeam_channel::TryRecvError::Empty => Some(0.0),
                crossbeam_channel::TryRecvError::Disconnected => None,
            },
        }
    }
}

impl AudioPlayback<f32> {
    pub fn new(sample_rate: u32) -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let config = supported_output_config(
            &device,
            StreamConfigQuery {
                sample_rate,
                buffer_size: 128,
                sample_format: cpal::SampleFormat::F32,
                channels: 1,
            },
        )?;

        debug!("using stream config {:?}", config);

        let (_stream, stream_handle) = OutputStream::try_from_device_config(&device, config)?;
        let sink = Sink::try_new(&stream_handle)?;

        let (mixer_handle, mixer_source) = mixer::<f32>(1, sample_rate);
        sink.append(mixer_source);
        sink.play();

        Ok(Self {
            _stream,
            _sink: Arc::new(sink),
            _mixer_handle: mixer_handle,
        })
    }

    pub fn sine(&self, freq: f32, duration: Duration) {
        self.play(SineWave::new(freq).take_duration(duration))
    }

    pub fn play<S>(&self, source: S)
    where
        S: Source<Item = f32> + Send + 'static,
    {
        self._mixer_handle.add(source);
    }

    pub fn new_output(&self, sample_rate: u32) -> Sender<f32> {
        let (tx1, rx1) = new_audio_channel();

        let out = AudioPlaybackOutput {
            _tx: tx1.clone(),
            rx: rx1,
            sample_rate,
        };

        self.play(out);

        tx1
    }
}
