use cpal::{SampleRate, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait};
use tracing::{error, trace};

pub fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    println!("host: {:?}", host.id().name());
    let devices = host.devices()?;
    println!("devices");
    for x in devices {
        if let Ok(name) = x.name() {
            println!("  - device: {}", name);

            match x.default_input_config() {
                Ok(x) => println!("    - in: {:?}", x),
                Err(e) => println!("    - in(ERR): {:?}", e),
            }

            match x.default_output_config() {
                Ok(x) => println!("    - out: {:?}", x),
                Err(e) => println!("    - out(ERR): {:?}", e),
            }
        }
    }

    // try to get default input device
    let device = host
        .default_input_device()
        .expect("No input device available");

    let config = StreamConfig {
        channels: 1,
        sample_rate: SampleRate(24_000),
        buffer_size: cpal::BufferSize::Default,
    };

    let err_fn = |err| eprintln!("Input stream error: {}", err);

    let _ = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for frame in data.chunks(1) {
                for &sample in frame {
                    trace!("{:?}", sample)
                }
            }
        },
        err_fn,
        None,
    )?;

    Ok(())
}
