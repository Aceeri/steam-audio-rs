use glam::Vec3;
use steam_audio::{prelude::*, simulation::simulation::SimulationSharedInputs, Orientation};

use std::error::Error;

const FILENAME: &'static str = "assets/eduardo.ogg";

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = Context::new(ContextSettings::default())?;
    let audio_settings = AudioSettings::default();
    let hrtf_settings = HRTFSettings::default();
    let hrtf = HRTF::new(&context, &audio_settings, &hrtf_settings)?;

    let audio = steam_audio::read_ogg(FILENAME)?;
    let audio_buffer = AudioBuffer::from_raw_pcm(&audio_settings, vec![audio]);

    let simulation_settings = SimulationSettings::from_audio_settings(&audio_settings);
    let mut simulator = Simulator::new(&mut context, &simulation_settings)?;

    let scene_settings = SceneSettings::default();
    let mut scene = Scene::new(&mut context, &scene_settings)?;

    let mesh_settings = StaticMeshSettings {
        vertices: vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ],
        triangles: vec![
            [0, 1, 2],
            [0, 2, 3],
        ],
        materials: vec![steam_audio::materials::GENERIC],
        material_indices: vec![0, 0],
    };

    dbg!();
    let static_mesh = StaticMesh::new(&scene, mesh_settings)?;
    scene.add_static_mesh(&static_mesh);
    dbg!();
    scene.commit();
    dbg!();

    context.debug();
    dbg!(unsafe { simulator.inner() });
    dbg!(unsafe { scene.inner() });
    simulator.set_scene(&scene);
    dbg!();
    simulator.commit();
    dbg!();

    let source_settings = &SourceSettings::default();
    let source = Source::new(&simulator, &source_settings)?;

    source.set_inputs(
        SimulationFlags::DIRECT,
        &SimulationInputs {
            source: Orientation {
                origin: Vec3::X,
                ..Default::default()
            },
            ..Default::default()
        },
    );
    dbg!();
    simulator.add_source(&source);
    simulator.set_shared_inputs(
        SimulationFlags::DIRECT,
        &SimulationSharedInputs {
            ..Default::default()
        },
    );
    dbg!();
    simulator.commit();
    dbg!();
    simulator.run_direct();
    dbg!();

    let outputs = source.get_outputs(SimulationFlags::DIRECT);
    dbg!(outputs);

    Ok(())
}
