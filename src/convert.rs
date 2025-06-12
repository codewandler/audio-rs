pub fn convert_f32_to_pcm16_bytes(samples: Vec<f32>) -> Vec<u8> {
    // to i16
    let pcm16: Vec<i16> = samples
        .iter()
        .map(|&sample| (sample.clamp(-1.0, 1.0) * i16::MAX as f32).round() as i16)
        .collect();
    // to u8
    pcm16
        .iter()
        .flat_map(|&s| s.to_le_bytes()) // Converts i16 to 2 u8 bytes
        .collect()
}

pub fn convert_pcm16_bytes_to_f32(samples: Vec<u8>) -> Vec<f32> {
    use std::convert::TryInto;

    // Ensure the input length is even
    let len = samples.len() / 2;
    let mut output = Vec::with_capacity(len);

    for chunk in samples.chunks_exact(2) {
        // Reconstruct i16 from two u8 bytes (little-endian)
        let bytes: [u8; 2] = chunk.try_into().unwrap();
        let sample = i16::from_le_bytes(bytes);
        // Convert i16 to f32 in the range [-1.0, 1.0]
        let float = sample as f32 / i16::MAX as f32;
        output.push(float);
    }

    output
}
