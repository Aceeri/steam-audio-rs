/// Interleave de-interleaved PCM data.
///
/// 111222333 -> 123123123
pub fn interleave(pcm: Vec<Vec<f32>>) -> Vec<f32> {
    let mut output = Vec::new();
    let length = match pcm.get(0) {
        Some(first_channel) => first_channel.len(),
        None => return output,
    };

    for index in 0..length {
        for channel in pcm.iter() {
            output.push(channel[index]);
        }
    }

    output
}

pub fn extend_deinterleaved(pcm: &mut Vec<Vec<f32>>, extend_with: &Vec<Vec<f32>>) {
    for (channel, pcm) in extend_with.iter().zip(pcm.iter_mut()) {
        pcm.extend(channel);
    }
}
