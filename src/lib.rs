mod buffer;
mod capture;
mod channel;
mod config;
mod convert;
mod ext;
mod format;
mod pipe;
mod playback;
mod source;

pub use {
    buffer::*, capture::*, channel::*, convert::*, format::*, pipe::*, playback::*, source::*,
};

pub trait AudioSink: Send {
    type Format: SampleFormat;

    fn audio_write(&self, d: Self::Format) -> anyhow::Result<()>;
}

pub trait AudioSource: Sync + Send {
    type Format: SampleFormat;

    fn audio_read(&mut self) -> Option<Self::Format>;
}
