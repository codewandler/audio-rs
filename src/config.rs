use cpal::traits::DeviceTrait;
use cpal::{SupportedStreamConfig, SupportedStreamConfigRange};
use rodio::cpal;

#[derive(Debug)]
pub struct StreamConfigQuery {
    pub sample_format: cpal::SampleFormat,
    pub channels: u16,
    pub buffer_size: u32,
    pub sample_rate: u32,
}

pub fn supported_input_config(
    device: &cpal::Device,
    q: StreamConfigQuery,
) -> anyhow::Result<SupportedStreamConfig> {
    find_best_config(q, device.supported_input_configs()?.into_iter().collect())
}

pub fn supported_output_config(
    device: &cpal::Device,
    q: StreamConfigQuery,
) -> anyhow::Result<SupportedStreamConfig> {
    find_best_config(q, device.supported_output_configs()?.into_iter().collect())
}

fn find_best_config(
    query: StreamConfigQuery,
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

            if !match c.buffer_size() {
                cpal::SupportedBufferSize::Unknown => false,
                cpal::SupportedBufferSize::Range { min, max } => {
                    return query.buffer_size.ge(min) && query.buffer_size.le(max);
                }
            } {
                return false;
            }

            if c.try_with_sample_rate(cpal::SampleRate(query.sample_rate))
                .is_none()
            {
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
