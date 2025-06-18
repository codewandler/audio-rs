use clap::Parser;
use rodio::cpal::SampleFormat;
use rodio::cpal::traits::HostTrait;
use rodio::DeviceTrait;
use codewandler_audio::{supported_input_config, supported_output_config, StreamConfigQuery};

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(short, long, default_value = "1024")]
    buffer_size: u32,

    #[clap(short, long, default_value = "24000")]
    sample_rate: u32,

    #[clap(short, long, default_value = "1")]
    channels: u16,
}

pub fn main() -> anyhow::Result<()> {
    let host = rodio::cpal::default_host();

    let args = Args::parse();

    let config_query = &StreamConfigQuery{
        buffer_size: args.buffer_size,
        sample_format: SampleFormat::F32,
        sample_rate: args.sample_rate,
        channels: args.channels,
    };

    println!("searching config for {:?}", config_query);

    let input_device = host.default_input_device().ok_or(anyhow::anyhow!("No default input device found"))?;
    println!("default(input): {:?}", input_device.default_input_config()?);
    match supported_input_config(&input_device, config_query) {
        Err(e) => println!("ERR(input): {}", e),
        Ok(config) => println!("OK(input): {:?}", config),
    }

    let output_device = host.default_output_device().ok_or(anyhow::anyhow!("No default output device found"))?;
    println!("default(output): {:?}", output_device.default_input_config()?);
    match supported_output_config(&output_device, config_query) {
        Err(e) => println!("ERR(output): {}", e),
        Ok(config) => println!("OK(output): {:?}", config),
    }

    Ok(())
}
