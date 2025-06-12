use crate::{AudioSource, SampleFormat};
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::sync::{Arc, Mutex};
use std::thread;

pub trait IntoAudioSource {
    type Format: SampleFormat;
    fn into_audio_source(&mut self) -> anyhow::Result<Box<dyn AudioSource<Format = Self::Format>>>;
}

pub struct AudioSourceFanOut<F, K>
where
    F: SampleFormat,
    K: IntoAudioSource,
{
    _k: K,
    subscribers: Arc<Mutex<Vec<Sender<F>>>>,
}

impl<F, K> AudioSourceFanOut<F, K>
where
    F: SampleFormat + 'static,
    K: IntoAudioSource<Format = F>,
{
    pub fn new(mut k: K) -> Self {
        let mut source = k.into_audio_source().unwrap();
        let subscribers = Arc::new(Mutex::new(Vec::<Sender<F>>::new()));
        let subs_clone = Arc::clone(&subscribers);

        // Fan-out thread
        thread::spawn(move || {
            while let Some(sample) = source.audio_read() {
                let mut disconnected = vec![];
                let mut subs = subs_clone.lock().unwrap();
                for (i, tx) in subs.iter().enumerate() {
                    if tx.send(sample).is_err() {
                        disconnected.push(i);
                    }
                }
                // Remove any dropped subscribers
                for i in disconnected.into_iter().rev() {
                    subs.remove(i);
                }
            }
        });

        Self { _k: k, subscribers }
    }

    pub fn subscribe(&self) -> Receiver<F> {
        let (tx, rx) = unbounded();
        self.subscribers.lock().unwrap().push(tx);
        rx
    }
}
