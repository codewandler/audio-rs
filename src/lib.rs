mod buffer;
mod capture;
mod channel;
mod convert;
mod ext;
mod format;
mod playback;
mod source;
mod stream;

pub use {
    buffer::*, capture::*, channel::*, convert::*, format::*, playback::*, source::*, stream::*,
};

pub trait AudioSink: Send {
    type Format: SampleFormat;

    fn audio_write(&self, d: Self::Format) -> anyhow::Result<()>;
}

pub trait AudioSource: Sync + Send {
    type Format: SampleFormat;

    fn audio_read(&mut self) -> Option<Self::Format>;
}
