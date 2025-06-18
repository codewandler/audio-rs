use cpal::traits::DeviceTrait;
use cpal::{SupportedStreamConfig, SupportedStreamConfigRange};
use rodio::cpal;

#[derive(Debug)]
pub struct StreamConfigQuery {
    pub sample_format: cpal::SampleFormat,
    pub channels: u16,
    pub sample_rate: u32,
}

pub fn supported_input_config(
    device: &cpal::Device,
    q: &StreamConfigQuery,
) -> anyhow::Result<SupportedStreamConfig> {
    find_best_config(q, device.supported_input_configs()?.into_iter().collect())
        .map_err(|e| anyhow::anyhow!("no supported audio input config: {:?}", e))
}

pub fn supported_output_config(
    device: &cpal::Device,
    q: &StreamConfigQuery,
) -> anyhow::Result<SupportedStreamConfig> {
    find_best_config(q, device.supported_output_configs()?.into_iter().collect())
        .map_err(|e| anyhow::anyhow!("no supported audio output config: {:?}", e))
}

fn find_best_config(
    query: &StreamConfigQuery,
    source: Vec<SupportedStreamConfigRange>,
) -> anyhow::Result<SupportedStreamConfig> {
    let matches = source
        .iter()
        .filter(|c| {
            if !c.sample_format().eq(&query.sample_format) {
                return false;
            }

            if !c.channels().eq(&query.channels) {
                return false;
            }

            true
        })
        .filter_map(|c| c.try_with_sample_rate(cpal::SampleRate(query.sample_rate)))
        .collect::<Vec<SupportedStreamConfig>>();

    match matches.first() {
        Some(found) => Ok(found.clone()),
        None => Err(anyhow::anyhow!("No supported config found: {:?}", query)),
    }
}
