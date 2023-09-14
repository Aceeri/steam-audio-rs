//
// Copyright 2017 Valve Corporation. All rights reserved. Subject to the following license:
// https://valvesoftware.github.io/steam-audio/license.html
//

#pragma once

#if defined(IPL_OS_WINDOWS)
#include "Windows.h"
#elif defined(IPL_OS_MACOSX)
#endif

#include "phonon.h"
#include "fmod.h"

#include "steamaudio_fmod_version.h"

// This function is called by FMOD Studio when it loads plugins. It returns metadata that describes all of the
// effects implemented in this DLL.
F_EXPORT FMOD_PLUGINLIST* F_CALL FMODGetPluginDescriptionList();

/**
 *  Returns the version of the FMOD Studio integration being used.
 * 
 *  \param  major   Major version number. For example, "1" in "1.2.3".
 *  \param  minor   Minor version number. For example, "2" in "1.2.3".
 *  \param  patch   Patch version number. For example, "3" in "1.2.3".
 */
F_EXPORT void F_CALL iplFMODGetVersion(unsigned int* major, unsigned int* minor, unsigned int* patch);

/**
 *  Initializes the FMOD Studio integration. This function must be called before creating any Steam Audio DSP effects.
 * 
 *  \param  context The Steam Audio context created by the game engine when initializing Steam Audio.
 */
F_EXPORT void F_CALL iplFMODInitialize(IPLContext context);

/**
 *  Shuts down the FMOD Studio integration. This function must be called after all Steam Audio DSP effects have been
 *  destroyed.
 */
F_EXPORT void F_CALL iplFMODTerminate();

/**
 *  Specifies the HRTF to use for spatialization in subsequent audio frames. This function must be called once during
 *  initialization, after \c iplFMODInitialize. It should also be called whenever the game engine needs to change the
 *  HRTF.
 * 
 *  \param  hrtf    The HRTF to use for spatialization.
 */
F_EXPORT void F_CALL iplFMODSetHRTF(IPLHRTF hrtf);

/**
 *  Specifies the simulation settings used by the game engine for simulating direct and/or indirect sound propagation.
 *  This function must be called once during initialization, after \c iplFMODInitialize.
 * 
 *  \param  simulationSettings  The simulation settings used by the game engine.
 */
F_EXPORT void F_CALL iplFMODSetSimulationSettings(IPLSimulationSettings simulationSettings);

/**
 *  Specifies the \c IPLSource object used by the game engine for simulating reverb. Typically, listener-centric reverb
 *  is simulated by creating an \c IPLSource object with the same position as the listener, and simulating reflections.
 *  To render this simulated reverb, call this function and pass it the \c IPLSource object used.
 * 
 *  \param  reverbSource    The source object used by the game engine for simulating reverb.
 */
F_EXPORT void F_CALL iplFMODSetReverbSource(IPLSource reverbSource);

F_EXPORT IPLint32 F_CALL iplFMODAddSource(IPLSource source);

F_EXPORT void F_CALL iplFMODRemoveSource(IPLint32 handle);
