use crate::format::SampleFormat;
use crate::{AudioSink, AudioSource, IntoAudioSource};
use crossbeam_channel::{Receiver, Sender, bounded};
use tracing::error;

impl<T> AudioSink for Sender<T>
where
    T: SampleFormat,
{
    type Format = T;

    fn audio_write(&self, d: Self::Format) -> anyhow::Result<()> {
        match self.send(d) {
            Ok(_) => (),
            Err(err) => {
                error!("Error sending audio buffer: {}", err);
                return Err(anyhow::anyhow!("Error sending audio buffer: {}", err));
            }
        }
        Ok(())
    }
}

impl<T> AudioSource for Receiver<T>
where
    T: SampleFormat,
{
    type Format = T;

    fn audio_read(&mut self) -> Option<Self::Format> {
        self.recv().ok()
    }
}

impl IntoAudioSource for Receiver<f32> {
    type Format = f32;

    fn into_audio_source(&mut self) -> anyhow::Result<Box<dyn AudioSource<Format = Self::Format>>> {
        Ok(Box::new(self.clone()))
    }
}

pub fn new_audio_channel<T: SampleFormat>() -> (Sender<T>, Receiver<T>) {
    bounded::<T>(64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::BufferWriter;
    use crate::{Buffer, audio_pipe};
    use std::thread;

    #[test]
    fn test_channel() {
        let (tx1, rx1) = new_audio_channel::<f32>();
        let (tx2, rx2) = new_audio_channel::<f32>();

        tx1.audio_write_buffer(&Buffer::new(vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
        ]))
        .unwrap();

        let h = thread::spawn(move || {
            audio_pipe(Box::new(rx1), Box::new(tx2)).unwrap();
        });

        h.join().unwrap();

        let x: Vec<f32> = rx2.iter().collect();
        println!("{:?}", x);
    }
}
