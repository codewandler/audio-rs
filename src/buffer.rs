use crate::AudioSink;
use crate::format::SampleFormat;

pub struct Buffer<T: SampleFormat>(Vec<T>);

impl<T> Buffer<T>
where
    T: SampleFormat,
{
    pub fn new(data: Vec<T>) -> Self {
        Self(data)
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }
}

pub trait BufferWriter<T>
where
    T: SampleFormat,
{
    fn audio_write_buffer(&self, buffer: &Buffer<T>) -> anyhow::Result<()>;
}

impl<F, T> BufferWriter<F> for T
where
    F: SampleFormat,
    T: AudioSink<Format = F>,
{
    fn audio_write_buffer(&self, buffer: &Buffer<F>) -> anyhow::Result<()> {
        for sample in buffer.iter() {
            self.audio_write(*sample)?;
        }
        Ok(())
    }
}
