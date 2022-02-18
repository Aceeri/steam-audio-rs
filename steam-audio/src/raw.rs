use std::{error::Error, fs::File, io::Write, path::Path};

use lewton::inside_ogg::OggStreamReader;

pub fn read_ogg<P: AsRef<Path>>(path: P) -> Result<Vec<f32>, Box<dyn Error>> {
    let file = File::open(path.as_ref())?;
    let mut stream_reader = OggStreamReader::new(file)?;
    assert_eq!(stream_reader.ident_hdr.audio_channels, 1);

    let mut concatted = Vec::new();
    while let Some(packet) = stream_reader.read_dec_packet_generic::<Vec<Vec<f32>>>()? {
        concatted.extend(packet[0].clone());
    }

    Ok(concatted)
}

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub fn write_file<P: AsRef<Path>>(path: P, data: Vec<f32>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write(vf_to_u8(&data))?;
    Ok(())
}

pub fn write_file_deitl<P: AsRef<Path>>(
    path: P,
    data: Vec<Vec<f32>>,
) -> Result<(), Box<dyn Error>> {
    write_file(path, crate::interleave(data))
}
