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

    let mut simulation_settings = SimulationSettings::from_audio_settings(&audio_settings);
    simulation_settings.flags = SimulationFlags::all();
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

    let static_mesh = StaticMesh::new(&scene, mesh_settings)?;
    scene.add_static_mesh(&static_mesh);

    scene.commit();

    simulator.set_scene(&scene);
    simulator.commit();

    let source_settings = &SourceSettings::default();
    let source = Source::new(&simulator, &source_settings)?;

    source.set_inputs(
        SimulationFlags::all(),
        &SimulationInputs {
            flags: SimulationFlags::all(),
            direct_flags: DirectSimulationFlags::all(),
            //occlusion_type: OcclusionType::Raycast,
            occlusion_radius: 0.0,
            source: Orientation {
                origin: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    simulator.add_source(&source);
    simulator.set_shared_inputs(
        SimulationFlags::all(),
        &SimulationSharedInputs {
            listener: Orientation {
                origin: Vec3::new(0.5, 0.5, -1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    dbg!();
    simulator.commit();
    dbg!();
    simulator.run_direct();
    dbg!();

    let outputs = source.get_outputs(SimulationFlags::all());
    dbg!(outputs);

    Ok(())
}
