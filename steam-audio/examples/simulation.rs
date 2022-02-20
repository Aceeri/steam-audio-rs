use glam::Vec3;
use steam_audio::{prelude::*, simulation::simulation::SimulationSharedInputs, Orientation};

use std::{error::Error, path::Path};

const FILENAME: &'static str = "assets/eduardo.ogg";

fn file_stem<P: AsRef<Path>>(p: P) -> String {
    p.as_ref()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn raw_to_file(
    kind: &'static str,
    name: String,
    data: Vec<Vec<f32>>,
) -> Result<(), Box<dyn Error>> {
    let out_name = format!("assets/out/{}/{}.raw", kind, name);
    println!("outputting to `{}`", out_name);
    let interleaved = steam_audio::interleave(data);
    steam_audio::write_file(out_name, interleaved)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = Context::new(ContextSettings::default())?;
    let audio_settings = AudioSettings::default();
    let hrtf_settings = HRTFSettings::default();
    let hrtf = HRTF::new(&context, &audio_settings, &hrtf_settings)?;

    let audio = steam_audio::read_ogg(FILENAME)?;
    let audio_buffer = AudioBuffer::from_raw_pcm(&audio_settings, vec![audio]);

    let mut simulation_settings = SimulationSettings::from_audio_settings(&audio_settings);
    simulation_settings.flags = SimulationFlags::all();
    let mut simulator = Simulator::new(&mut context, &simulation_settings)?;

    let scene_settings = SceneSettings::default();
    let mut scene = Scene::new(&mut context, &scene_settings)?;

    let mesh_settings = StaticMeshSettings {
        vertices: vec![
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(1.0, 0.0, 0.5),
            Vec3::new(1.0, 1.0, 0.5),
            Vec3::new(0.0, 1.0, 0.5),
        ],
        triangles: vec![
            [0, 1, 2],
            [0, 2, 3],
        ],
        materials: vec![steam_audio::materials::GENERIC],
        material_indices: vec![0, 0],
    };

    let static_mesh = StaticMesh::new(&scene, mesh_settings)?;
    scene.add_static_mesh(&static_mesh);

    scene.commit();

    simulator.set_scene(&scene);
    simulator.commit();

    let source_settings = &SourceSettings::default();
    let source = Source::new(&simulator, &source_settings)?;

    simulator.set_shared_inputs(
        SimulationFlags::all(),
        &SimulationSharedInputs {
            listener: Orientation {
                origin: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    );


    simulator.add_source(&source);
    dbg!();
    simulator.commit();
    dbg!();
    dbg!();

    let outputs = source.get_outputs(SimulationFlags::all());
    dbg!(outputs);

    let mut output: Vec<Vec<f32>> = vec![vec![]; 2];
    let mut binaural_output: Vec<Vec<f32>> = vec![vec![]; 2];
    let mut binaural_output_buffer = AudioBuffer::frame_buffer_with_channels(&audio_settings, 2);
    let mut direct_output_buffer = AudioBuffer::frame_buffer_with_channels(&audio_settings, 2);
    let frame_length = audio_buffer.frames();

    let direct_effect = DirectEffect::new(&context, &audio_settings, 2)?;
    let binaural_effect = BinauralEffect::new(&context, &audio_settings, &hrtf)?;
    for (frame_index, frame) in audio_buffer.into_iter().enumerate() {
        let time = (frame_index as f32 / frame_length as f32) * std::f32::consts::TAU * 5.0;

        let mut params = BinauralParams::default();
        params.interpolation = HRTFInterpolation::Bilinear;
        params.direction = Vec3::new(time.cos(), 0.0, time.sin());

        binaural_effect.apply_to_buffer(&params, frame, &mut binaural_output_buffer)?;
        let (ptrs, binaural_frame) = binaural_output_buffer.current_frame();

        steam_audio::extend_deinterleaved(&mut binaural_output, &binaural_output_buffer.data);

        source.set_inputs(
            SimulationFlags::all(),
            &SimulationInputs {
                flags: SimulationFlags::all(),
                direct_flags: DirectSimulationFlags::all(),
                //occlusion_type: OcclusionType::Raycast,
                occlusion_radius: 0.0,
                source: Orientation {
                    origin: Vec3::new(time.cos(), 0.0, time.sin()),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        simulator.commit();
        simulator.run_direct();
        let outputs = source.get_outputs(SimulationFlags::all());
        let direct = DirectEffectParams::default();

        direct_effect.apply_to_buffer(&direct, binaural_frame, &mut direct_output_buffer)?;
        steam_audio::extend_deinterleaved(&mut output, &direct_output_buffer.data);
    }

    let filestem = file_stem(FILENAME);
    raw_to_file("simulation", filestem.clone(), output)?;
    raw_to_file("binaural", filestem, binaural_output)?;

    Ok(())
}
