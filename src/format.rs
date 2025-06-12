pub trait SampleFormat: rodio::Sample + Sync + Send {}

impl SampleFormat for f32 {}
impl SampleFormat for i16 {}

pub type FormatPCM16 = i16;
