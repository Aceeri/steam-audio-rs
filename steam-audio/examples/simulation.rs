use steam_audio::{prelude::*, simulation::source::SourceSettings};

use std::error::Error;

const FILENAME: &'static str = "assets/eduardo.ogg";

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new(ContextSettings::default())?;
    let audio_settings = AudioSettings::default();
    let hrtf_settings = HRTFSettings::default();
    let hrtf = HRTF::new(&context, &audio_settings, &hrtf_settings)?;

    let audio = steam_audio::read_ogg(FILENAME)?;
    let audio_buffer = AudioBuffer::from_raw_pcm(&audio_settings, vec![audio]);
    let simulation_settings = SimulationSettings::from_audio_settings(&audio_settings);
    let simulator = Simulator::new(&context, &simulation_settings)?;

    let source_settings = &SourceSettings::default();
    let source = Source::new(&simulator, &source_settings);

    Ok(())
}
