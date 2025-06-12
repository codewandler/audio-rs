use codewandler_audio::{AudioPlayback, audio_capture, audio_pipe};
use tokio::signal::ctrl_c;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pb = AudioPlayback::new(24_000)?;
    //pb.sine(1440.0, Duration::from_millis(1000));

    // play via buffer
    //let out = pb.new_output(48_000);

    //out.audio_write_buffer(&Buffer::new(SineWave::new(440.0).take_duration(Duration::from_millis(5_000)).collect()))?;
    //out.audio_write_buffer(&Buffer::new(SineWave::new(240.0).take_duration(Duration::from_millis(8_000)).collect()))?;

    // capture
    let cap_out = pb.new_output(24_000);

    let cap = audio_capture(24_000)?;
    let cap_rx = cap.subscribe();
    audio_pipe(Box::new(cap_rx), Box::new(cap_out))?;

    // wait
    ctrl_c().await.expect("TODO: panic message");
    Ok(())
}
