use steam_audio::prelude::*;
use steam_audio::simulation::simulation::{ReflectionEffectType, SceneType};
use steam_audio_sys::ffi::*;

fn main() {
    let simulation_settings = SimulationSettings {
        flags: SimulationFlags::DIRECT,
        scene_type: SceneType::Default,
        reflection_type: ReflectionEffectType::Convolution,
        max_num_occlusion_samples: 0,
        max_num_rays: 4096,
        num_diffuse_samples: 32,
        max_duration: 1.0,
        max_order: 1,
        max_num_sources: 256,
        num_threads: 2,
        ray_batch_size: 0,
        num_vis_samples: 0,
        sampling_rate: 44100,
        frame_size: 1024,
    };

    let source_settings = SourceSettings {
        flags: SimulationFlags::DIRECT,
    };

    let mut context_settings = IPLContextSettings {
        version: STEAMAUDIO_VERSION,
        logCallback: None,
        allocateCallback: None,
        simdLevel: IPLSIMDLevel::IPL_SIMDLEVEL_SSE2,
        freeCallback: None,
    };

    unsafe {
        let mut context: IPLContext = std::mem::zeroed();
        let mut simulator: IPLSimulator = std::mem::zeroed();
        let mut source: IPLSource = std::mem::zeroed();

        let mut simulation_settings: IPLSimulationSettings = (&simulation_settings).into();
        let mut source_settings: IPLSourceSettings = (&source_settings).into();

        dbg!(context);
        iplContextCreate(&mut context_settings, &mut context);
        dbg!(context);

        //match ffi::iplSimulatorCreate(context.inner, &mut simulator.settings, &mut simulator.inner) {
        dbg!(simulator);
        iplSimulatorCreate(context, &mut simulation_settings, &mut simulator);
        dbg!(simulator);

        iplSimulatorCommit(simulator);

        dbg!(source);
        iplSourceCreate(simulator, &mut source_settings, &mut source);
        dbg!(source);
    };
}
