use crate::format::SampleFormat;
use crate::{AudioSink, AudioSource};
use std::fmt::Debug;

pub fn audio_pipe<F: SampleFormat + Debug>(
    mut source: Box<dyn AudioSource<Format = F>>,
    sink: Box<dyn AudioSink<Format = F>>,
) -> anyhow::Result<()> {
    while let Some(d) = source.audio_read() {
        sink.audio_write(d)?;
    }
    Ok(())
}
