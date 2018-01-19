extern crate libc;

use super::types::*;

#[cfg(all(target_os = "linux"))]
#[link(name = "phonon")]
extern "C" {}

///*****************************************************************************************************************/
///* Context                                                                                                       */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Context object. A Context object must be created before creating any other API objects.
    ///
    /// \param  logCallback         Callback for logging messages. Can be NULL.
    /// \param  allocateCallback    Callback for allocating memory. Can be NULL.
    /// \param  freeCallback        Callback for freeing memory. Can be NULL.
    /// \param  context             [out] Handle to the created Context object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateContext(
        logCallback: IPLLogFunction,
        allocateCallback: IPLAllocateFunction,
        freeCallback: IPLFreeFunction,
        context: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Context object. If any other API objects are still referencing the Context object, it will not be
    /// destroyed; destruction occurs when the Context object's reference count reaches zero.
    ///
    /// \param  context             [in, out] Address of a handle to the Context object to destroy.
    ///
    pub fn iplDestroyContext(context: *mut IPLhandle);

    /// Performs last-minute cleanup and finalization. This function must be the last API function to be called before
    /// your application exits.
    ///
    pub fn iplCleanup();
}

///*****************************************************************************************************************/
///* Geometry                                                                                                      */
///*****************************************************************************************************************/

extern "C" {
    /// Calculates the relative direction from the listener to a sound source. The returned direction
    /// vector is expressed in the listener's coordinate system.
    ///
    /// \param  sourcePosition      World-space coordinates of the source.
    /// \param  listenerPosition    World-space coordinates of the listener.
    /// \param  listenerAhead       World-space unit-length vector pointing ahead relative to the listener.
    /// \param  listenerUp          World-space unit-length vector pointing up relative to the listener.
    ///
    /// \return A unit-length vector in the listener's coordinate space, pointing from the listener to the source.
    ///
    pub fn iplCalculateRelativeDirection(
        sourcePosition: IPLVector3,
        listenerPosition: IPLVector3,
        listenerAhead: IPLVector3,
        listenerUp: IPLVector3,
    ) -> IPLVector3;
}

///*****************************************************************************************************************/
///* OpenCL Compute Devices                                                                                        */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Compute Device object. The same Compute Device must be used by the game engine and audio engine
    /// parts of the Phonon integration. Depending on the OpenCL driver and device, this function may take some
    /// time to execute, so do not call it from performance-sensitive code.
    ///
    /// \param  context         The Context object used by the game engine.
    /// \param	deviceFilter    Constraints on the type of device to create.
    /// \param  device          [out] Handle to the created Compute Device object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateComputeDevice(
        context: IPLhandle,
        deviceFilter: IPLComputeDeviceFilter,
        device: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Compute Device object. If any other API objects are still referencing the Compute Device object,
    /// it will not be destroyed; destruction occurs when the object's reference count reaches zero.
    ///
    /// \param  device  [in, out] Address of a handle to the Compute Device object to destroy.
    ///
    pub fn iplDestroyComputeDevice(device: *mut IPLhandle);
}

///*****************************************************************************************************************/
///* Scene                                                                                                         */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Scene object. A Scene object does not store any geometry information on its own; for that you
    /// need to create one or more Static Mesh objects and add them to the Scene object. The Scene object
    /// does contain an array of materials; all triangles in all Static Mesh objects refer to this array in order
    /// to specify their material properties.
    ///
    /// \param  context             The Context object used by the game engine.
    /// \param  computeDevice       Handle to a Compute Device object. Only required if using Radeon Rays for
    ///                             ray tracing, may be \c NULL otherwise.
    /// \param  simulationSettings  The settings to use for simulation.
    /// \param  numMaterials        The number of materials that are used to describe the various surfaces in
    ///                             the scene. Materials may not be added or removed once the Scene object is
    ///                             created.
    /// \param  scene               [out] Handle to the created Scene object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateScene(
        context: IPLhandle,
        computeDevice: IPLhandle,
        simulationSettings: IPLSimulationSettings,
        numMaterials: IPLint32,
        scene: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Scene object. If any other API objects are still referencing the Scene object, it will not be
    /// destroyed; destruction occurs when the object's reference count reaches zero.
    ///
    /// \param  scene               [in, out] Address of a handle to the Scene object to destroy.
    ///
    pub fn iplDestroyScene(scene: *mut IPLhandle);

    /// Specifies a single material used by a Scene object. All materials must be completely specified before
    /// simulation occurs, otherwise simulation results will be incorrect.
    ///
    /// \param  scene               Handle to the Scene object.
    /// \param  materialIndex       Index of the material to set. Between 0 and N-1, where N is the value of
    ///                             \c numMaterials passed to \c ::iplCreateScene.
    /// \param  material            The material properties to use.
    ///
    pub fn iplSetSceneMaterial(scene: IPLhandle, materialIndex: IPLint32, material: IPLMaterial);

    /// Specifies callbacks that allow a Scene object to call into a user-specified custom ray tracer. This function
    /// should only be called if using a custom ray tracer, or else undefined behavior will occur. When using a custom
    /// ray tracer, this function must be called before any simulation occurs, otherwise undefined behavior will
    /// occur.
    ///
    /// \param  scene               Handle to the Scene object.
    /// \param  closestHitCallback  Pointer to a function that returns the closest hit along a ray.
    /// \param  anyHitCallback      Pointer to a function that returns whether a ray hits anything.
    /// \param  userData            Pointer to a block of memory containing arbitrary data for use
    ///                             by the closest hit and any hit callbacks.
    ///
    pub fn iplSetRayTracerCallbacks(
        scene: IPLhandle,
        closestHitCallback: IPLClosestHitCallback,
        anyHitCallback: IPLAnyHitCallback,
        userData: *mut IPLvoid,
    );

    /// Creates a Static Mesh object. A Static Mesh object represents a triangle mesh that does not change after it
    /// is created. A Static Mesh object also contains a mapping between each of its triangles and their acoustic
    /// material properties. Static Mesh objects should be used for scene geometry that is guaranteed to never change,
    /// such as rooms, buildings, or triangulated terrain. A Scene object may contain multiple Static Mesh objects,
    /// although typically one is sufficient.
    ///
    /// \param  scene               Handle to the Scene object to which to add the Static Mesh object.
    /// \param  numVertices         Number of vertices in the triangle mesh.
    /// \param  numTriangles        Number of triangles in the triangle mesh.
    /// \param  staticMesh          [out] Handle to the created Static Mesh object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateStaticMesh(
        scene: IPLhandle,
        numVertices: IPLint32,
        numTriangles: IPLint32,
        staticMesh: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Static Mesh object. If any other API objects are still referencing the Static Mesh object, it will
    /// not be destroyed; destruction occurs when the object's reference count reaches zero. Since the Scene object
    /// maintains an internal reference to the Static Mesh object, you may call this function at any point after
    /// fully specifying the Static Mesh object using \c ::iplSetStaticMeshVertices, \c ::iplSetStaticMeshTriangles,
    /// and \c ::iplSetStaticMeshMaterials.
    ///
    /// \param  staticMesh          [in, out] Address of a handle to the Static Mesh object to destroy.
    ///
    pub fn iplDestroyStaticMesh(staticMesh: *mut IPLhandle);

    /// Specifies the vertices of a Static Mesh object. All vertices must be converted from the game engine's
    /// coordinate system to Phonon's coordinate system before being passed to this function.
    ///
    /// \param  scene               Handle to the Scene object containing the Static Mesh object.
    /// \param  staticMesh          Handle to the Static Mesh object.
    /// \param  vertices            Array containing the coordinates of all vertices in the Static Mesh object.
    ///                             The number of \c IPLVector3 objects in the array must be equal to the value of
    ///                             \c numVertices passed to \c ::iplCreateStaticMesh.
    ///
    pub fn iplSetStaticMeshVertices(
        scene: IPLhandle,
        staticMesh: IPLhandle,
        vertices: *mut IPLVector3,
    );

    /// Specifies the triangles of a Static Mesh object. Triangle indices passed using this function refer to
    /// the vertex array passed using \c ::iplSetStaticMeshVertices.
    ///
    /// \param  scene               Handle to the Scene object containing the Static Mesh object.
    /// \param  staticMesh          Handle to the Static Mesh object.
    /// \param  triangles           Array containing all triangles in the Static Mesh object. The number of
    ///                             \c IPLTriangle objects in the array must be equal to the value of
    ///                             \c numTriangles passed to \c ::iplCreateStaticMesh.
    ///
    pub fn iplSetStaticMeshTriangles(
        scene: IPLhandle,
        staticMesh: IPLhandle,
        triangles: *mut IPLTriangle,
    );

    /// Specifies the materials associated with each triangle in a Static Mesh object. Material indices passed
    /// using this function refer to the array containing material data passed to \c ::iplSetSceneMaterial.
    ///
    /// \param  scene               Handle to the Scene object containing the Static Mesh object.
    /// \param  staticMesh          Handle to the Static Mesh object.
    /// \param  materialIndices     Array containing material indices for all triangles in the Static Mesh object.
    ///                             The number of material indices in the array must be equal to the value of
    ///                             \c numTriangles passed to \c ::iplCreateStaticMesh.
    ///
    pub fn iplSetStaticMeshMaterials(
        scene: IPLhandle,
        staticMesh: IPLhandle,
        materialIndices: *mut IPLint32,
    );

    /// Finalizes a scene and builds internal data structures. Once this function is called, you may not modify
    /// the Scene object or any Static Mesh objects it contains in any way. This function results in various
    /// internal data structures being generated; if using Radeon Rays, it results in scene data being uploaded
    /// to the GPU. This is a time-consuming, blocking call, so do not call it from performance-sensitive code.
    ///
    /// \param  scene               Handle to the Scene object.
    /// \param  progressCallback    Pointer to a function that reports the percentage of this function's work
    ///                             that has been completed. May be \c NULL.
    ///
    pub fn iplFinalizeScene(scene: IPLhandle, progressCallback: IPLFinalizeSceneProgressCallback);

    /// Serializes a Scene object to a byte array. The \c ::iplFinalizeScene function must have been called on
    /// the Scene object before calling this function. This function can only be called on a Scene object that
    /// has been created using the Phonon built-in ray tracer.
    ///
    /// \param  scene               Handle to the Scene object.
    /// \param  data                [out] Byte array into which the Scene object will be serialized. It is the
    ///                             caller's responsibility to manage memory for this array. The array must be large
    ///                             enough to hold all the data in the Scene object. May be \c NULL, in which case
    ///                             no data is returned; this is useful when finding out the size of the data stored
    ///                             in the Scene object.
    ///
    pub fn iplSaveFinalizedScene(scene: IPLhandle, data: *mut IPLbyte) -> IPLint32;

    /// Creates a Scene object based on data stored in a byte array. After this function is called, it is not
    /// necessary to call \c ::iplFinalizeScene on the resulting Scene object.
    ///
    /// \param  context             The Context object used by the game engine.
    /// \param  simulationSettings  The settings to use for the simulation. This must exactly match the settings
    ///                             that were used to create the original Scene object that was passed to
    ///                             \c ::iplSaveFinalizedScene, except for the \c sceneType and \c simulationType
    ///                             data members. This allows you to use the same file to create a Scene object
    ///                             that uses any ray tracer you prefer.
    /// \param  data                Byte array containing the serialized representation of the Scene object. Must
    ///                             not be \c NULL.
    /// \param  size                Size (in bytes) of the serialized data.
    /// \param  computeDevice       Handle to a Compute Device object. Only required if using Radeon Rays for
    ///                             ray tracing, may be \c NULL otherwise.
    /// \param  progressCallback    Pointer to a function that reports the percentage of this function's work
    ///                             that has been completed. May be \c NULL.
    /// \param  scene               [out] Handle to the created Scene object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplLoadFinalizedScene(
        context: IPLhandle,
        simulationSettings: IPLSimulationSettings,
        data: *mut IPLbyte,
        size: IPLint32,
        computeDevice: IPLhandle,
        progressCallback: IPLLoadSceneProgressCallback,
        scene: *mut IPLhandle,
    ) -> IPLerror;

    /// Saves a Scene object to an OBJ file. An OBJ file is a widely-supported 3D model file format, that can be
    /// displayed using a variety of software on most PC platforms. The OBJ file generated by this function can be
    /// useful for detecting problems that occur when exporting scene data from the game engine to Phonon. The
    /// \c ::iplFinalizeScene function must have been called on the Scene object before calling this function.
    /// This function can only be called on a Scene object that has been created using the Phonon built-in ray tracer.
    ///
    /// \param  scene               Handle to the Scene object.
    /// \param  fileBaseName        Absolute or relative path to the OBJ file to generate.
    ///
    pub fn iplDumpSceneToObjFile(scene: IPLhandle, fileBaseName: IPLstring);
}

///*****************************************************************************************************************/
///* Environment                                                                                                   */
///*****************************************************************************************************************/

extern "C" {
    /// Creates an Environment object. It is necessary to call this function even if you are not using the sound
    /// propagation features of Phonon.
    ///
    /// \param  context             The Context object used by the game engine.
    /// \param  computeDevice       Handle to a Compute Device object. Only required if using Radeon Rays for
    ///                             ray tracing, or if using TrueAudio Next for convolution, may be \c NULL otherwise.
    /// \param  simulationSettings  The settings to use for simulation. This must be the same settings passed to
    ///                             \c ::iplCreateScene or \c ::iplLoadFinalizedScene, whichever was used to create
    ///                             the Scene object passed in the \c scene parameter to this function.
    /// \param  scene               The Scene object. If created using \c ::iplCreateScene, then \c ::iplFinalizeScene
    ///                             must have been called on the Scene object before passing it to this function.
    ///                             May be \c NULL, in which case only direct sound will be simulated, without
    ///                             occlusion or any other indirect sound propagation.
    /// \param  probeManager        The Probe Manager object. May be \c NULL if not using baked data.
    /// \param  environment         [out] Handle to the created Environment object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateEnvironment(
        context: IPLhandle,
        computeDevice: IPLhandle,
        simulationSettings: IPLSimulationSettings,
        scene: IPLhandle,
        probeManager: IPLhandle,
        environment: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys an Environment object. If any other API objects are still referencing the Environment object, it will
    /// not be destroyed; destruction occurs when the object's reference count reaches zero.
    ///
    /// \param  environment         [in, out] Address of a handle to the Environment object to destroy.
    ///
    pub fn iplDestroyEnvironment(environment: *mut IPLhandle);

    /** Sets the number of bounces to use for real-time simulations that use an Environment object. Calling this
     *  function overrides the value of \c bounces set on the \c IPLSimulationSettings structure passed when
     *  calling \c ::iplCreateEnvironment to create this Environment object.
     *
     *  \param  environment         Handle to an Environment object.
     *  \param  numBounces          The number of bounces to use for all subsequent simulations in the Environment.
     */
    pub fn iplSetNumBounces(environment: IPLhandle, numBounces: IPLint32);
}

///*****************************************************************************************************************/
///* Audio Buffers                                                                                                 */
///*****************************************************************************************************************/

extern "C" {
    /// Mixes a set of audio buffers.  This is primarily useful for mixing the output of multiple Panning Effect
    /// objects, before passing them to a single Virtual Surround Effect or a single Ambisonics Binaural Effect. This
    /// way, applications can significantly accelerate 3D audio rendering for large numbers of sources.
    ///
    /// \param  numBuffers          The number of input buffers to mix. Must be greater than 0.
    /// \param  inputAudio          Array of audio buffers to mix. All of these audio buffers must have identical
    ///                             formats.
    /// \param  outputAudio         Audio buffer that will contain the mixed audio data. The format of this buffer
    ///                             must be identical to all buffers contained in \c inputAudio.
    ///
    pub fn iplMixAudioBuffers(
        numBuffers: IPLint32,
        inputAudio: *mut IPLAudioBuffer,
        outputAudio: IPLAudioBuffer,
    );

    /// Interleaves a deinterleaved audio buffer. The formats of \c inputAudio and \c outputAudio must be identical
    /// except for the \c channelOrder field.
    ///
    /// \param  inputAudio          The input audio buffer. This audio buffer must be deinterleaved.
    /// \param  outputAudio         The output audio buffer. This audio buffer must be interleaved.
    ///
    pub fn iplInterleaveAudioBuffer(inputAudio: IPLAudioBuffer, outputAudio: IPLAudioBuffer);

    /// Deinterleaves an interleaved audio buffer. The formats of \c inputAudio and \c outputAudio must be identical
    /// except for the \c channelOrder field.
    ///
    /// \param  inputAudio          The input audio buffer. This audio buffer must be interleaved.
    /// \param  outputAudio         The output audio buffer. This audio buffer must be deinterleaved.
    ///
    pub fn iplDeinterleaveAudioBuffer(inputAudio: IPLAudioBuffer, outputAudio: IPLAudioBuffer);

    /// Converts the format of an audio buffer into the format of the output audio buffer. This is primarily useful
    /// for 360 video and audio authoring workflows. The following format conversions are supported:
    ///
    /// - mono to multi-channel speaker-based formats (stereo, quadraphonic, 5.1, 7.1)
    /// - multi-channel speaker-based (stereo, quadraphonic, 5.1, 7.1) to mono
    /// - stereo to 5.1 or 7.1
    /// - Ambisonics to multi-channel speaker-based (mono, stereo, quadraphonic, 5.1, 7.1)
    ///
    /// \param  inputAudio          The input audio buffer.
    /// \param  outputAudio         The output audio buffer.
    ///
    pub fn iplConvertAudioBufferFormat(inputAudio: IPLAudioBuffer, outputAudio: IPLAudioBuffer);

    /// Creates an Ambisonics Rotator object. An Ambisonics Rotator object is used to apply an arbitrary rotation to
    /// audio data encoded in Ambisonics. This is primarily useful in the following situations:
    ///
    /// - If you have an Ambisonics audio buffer whose coefficients are defined relative to world space coordinates,
    ///   you can convert them to listener space using an Ambisonics Rotator object. This is necessary when using a
    ///   Convolution Effect object, since its output is defined in world space, and will not change if the listener
    ///   looks around.
    ///
    /// - If your final mix is encoded in Ambisonics, and the user is using headphones with head tracking, you can use
    ///   the Ambisonics Rotator object to make the sound field stay "in place" as the user looks around in the real
    ///   world. This is achieved by using the Ambisonics Rotator object to apply the inverse of the user's rotation
    ///   to the final mix.
    ///
    /// \param  context             The Context object used by the audio engine.
    /// \param  order               The order of the Ambisonics data to rotate.
    /// \param  rotator             [out] Handle to the created Ambisonics Rotator object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateAmbisonicsRotator(
        context: IPLhandle,
        order: IPLint32,
        rotator: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys an Ambisonics Rotator object.
    ///
    /// \param  rotator             [in, out] Address of a handle to the Ambisonics Rotator object to destroy.
    ///
    pub fn iplDestroyAmbisonicsRotator(rotator: *mut IPLhandle);

    /// Specifies a rotation value. This function must be called before using \c ::iplRotateAmbisonicsAudioBuffer to
    /// rotate an Ambisonics-encoded audio buffer, or the resulting audio will be incorrect.
    ///
    /// \param  rotator             Handle to an Ambisonics Rotator object.
    /// \param  quaternion          A unit quaternion describing the 3D transformation from world space to listener
    ///                             space coordinates.
    ///
    pub fn iplSetAmbisonicsRotation(rotator: IPLhandle, quaternion: IPLQuaternion);

    /// Rotates an Ambisonics-encoded audio buffer. The \c ::iplSetAmbisonicsRotation function must have been called
    /// prior to calling this function, or the resulting audio will be incorrect. It is possible to pass the same
    /// value for \c inputAudio and \c outputAudio. This results in in-place rotation of the Ambisonics data.
    ///
    /// \param  rotator             Handle to an Ambisonics Rotator object.
    /// \param  inputAudio          Audio buffer containing the Ambisonics-encoded data that is to be rotated. The
    ///                             format of this buffer must be Ambisonics.
    /// \param  outputAudio         Audio buffer containing the rotated Ambisonics-encoded data. The format of this
    ///                             buffer must be Ambisonics.
    ///
    pub fn iplRotateAmbisonicsAudioBuffer(
        rotator: IPLhandle,
        inputAudio: IPLAudioBuffer,
        outputAudio: IPLAudioBuffer,
    );
}

///*****************************************************************************************************************/
///* Binaural Renderer                                                                                             */
///*****************************************************************************************************************/

extern "C" {
    // Creates a Binaural Renderer object. This function must be called before creating any Panning Effect objects,
    // Object-Based Binaural Effect objects, Virtual Surround Effect objects, or Ambisonics Binaural Effect objects.
    // Calling this function for the first time is somewhat expensive; avoid creating Binaural Renderer objects in
    // your audio thread if at all possible. **This function is not thread-safe. It cannot be simultaneously called
    // from multiple threads.**
    //
    // \param  context             The Context object used by the audio engine.
    // \param  renderingSettings   An \c IPLRenderingSettings object describing the audio pipeline's DSP processing
    //                             parameters. These properties must remain constant throughout the lifetime of your
    //                             application.
    // \param  params              Parameters describing the type of HRTF data you wish to use (built-in HRTF data or
    //                             your own custom HRTF data).
    // \param  renderer            [out] Handle to the created Binaural Renderer object.
    //
    // \return Status code indicating whether or not the operation succeeded.
    //
    pub fn iplCreateBinauralRenderer(
        context: IPLhandle,
        renderingSettings: IPLRenderingSettings,
        params: IPLHrtfParams,
        renderer: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Binaural Renderer object. If any other API objects are still referencing the Binaural Renderer
    /// object, it will not be destroyed; destruction occurs when the object's reference count reaches zero.
    ///
    /// \param  renderer            [in, out] Address of a handle to the Binaural Renderer object to destroy.
    ///
    pub fn iplDestroyBinauralRenderer(renderer: *mut IPLhandle);
}

///*****************************************************************************************************************/
///* Panning Effect                                                                                                */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Panning Effect object. This can be used to render a point source on surround speakers, or using
    /// Ambisonics.
    ///
    /// \param  renderer            Handle to a Binaural Renderer object.
    /// \param  inputFormat         The format of the audio buffers that will be passed as input to this effect. All
    ///                             subsequent calls to \c ::iplApplyPanningEffect for this effect object must use
    ///                             \c IPLAudioBuffer objects with the same format as specified here. The input format
    ///                             must not be Ambisonics.
    /// \param  outputFormat        The format of the audio buffers which will be used to retrieve the output from
    ///                             this effect. All subsequent calls to \c ::iplApplyPanningEffect for this effect
    ///                             object must use \c IPLAudioBuffer objects with the same format as specified here.
    ///                             Any valid audio format may be specified as the output format.
    /// \param  effect              [out] Handle to the created Panning Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreatePanningEffect(
        renderer: IPLhandle,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Panning Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Panning Effect object to destroy.
    ///
    pub fn iplDestroyPanningEffect(effect: *mut IPLhandle);

    /// Applies 3D panning to a buffer of audio data, using the configuration of a Panning Effect object. The input
    /// audio is treated as emanating from a single point. If the input audio buffer contains more than one channel,
    /// it will automatically be downmixed to mono.
    ///
    /// \param  effect              Handle to a Panning Effect object.
    /// \param  inputAudio          Audio buffer containing the data to render using 3D panning. The format of this
    ///                             buffer must match the \c inputFormat parameter passed to \c ::iplCreatePanningEffect.
    /// \param  direction           Unit vector from the listener to the point source, relative to the listener's
    ///                             coordinate system.
    /// \param  outputAudio         Audio buffer that should contain the rendered audio data. The format of this buffer
    ///                             must match the \c outputFormat parameter passed to \c ::iplCreatePanningEffect.
    ///
    pub fn iplApplyPanningEffect(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        direction: IPLVector3,
        outputAudio: IPLAudioBuffer,
    );

    /** Resets any internal state maintained by a Panning Effect object. This is useful if the Panning Effect object
     *  is going to be disabled/unused for a few frames; resetting the internal state will prevent an audible glitch
     *  when the Panning Effect object is re-enabled at a later time.
     *
     *  \param  effect              Handle to a Panning Effect object.
     */
    pub fn iplFlushPanningEffect(effect: IPLhandle);
}

///*****************************************************************************************************************/
///* Object-Based Binaural Effect                                                                                  */
///*****************************************************************************************************************/

extern "C" {
    /// Creates an Object-Based Binaural Effect object. This can be used to render a point source using HRTF-based
    /// binaural rendering.
    ///
    /// \param  renderer            Handle to a Binaural Renderer object.
    /// \param  inputFormat         The format of the audio buffers that will be passed as input to this effect. All
    ///                             subsequent calls to \c ::iplApplyBinauralEffect for this effect object must use
    ///                             \c IPLAudioBuffer objects with the same format as specified here. The input format
    ///                             must not be Ambisonics.
    /// \param  outputFormat        The format of the audio buffers which will be used to retrieve the output from this
    ///                             effect. All subsequent calls to \c ::iplApplyBinauralEffect for this effect object
    ///                             must use \c IPLAudioBuffer objects with the same format as specified here. The
    ///                             output format must be stereo (2 channels).
    /// \param  effect              [out] Handle to the created Object-Based Binaural Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateBinauralEffect(
        renderer: IPLhandle,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys an Object-Based Binaural Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Object-Based Binaural Effect object to
    ///                             destroy.
    ///
    pub fn iplDestroyBinauralEffect(effect: *mut IPLhandle);

    /// Applies HRTF-based binaural rendering to a buffer of audio data. The input audio is treated as emanating from
    /// a single point. If the input audio buffer contains more than one channel, it will automatically be downmixed to
    /// mono. Using bilinear interpolation (by setting \c interpolation to \c ::IPL_HRTFINTERPOLATION_BILINEAR) can
    /// incur a relatively high CPU cost. Use it only on sources where nearest-neighbor filtering
    /// (\c ::IPL_HRTFINTERPOLATION_NEAREST) produces suboptimal results. Typically, bilinear filtering is most useful
    /// for wide-band noise-like sounds, such as radio static, mechanical noise, fire, etc.
    ///
    /// \param  effect              Handle to an Object-Based Binaural Effect object.
    /// \param  inputAudio          Audio buffer containing the data to render using binaural rendering. The format of
    ///                             this buffer must match the \c inputFormat parameter passed to
    ///                             \c ::iplCreateBinauralEffect.
    /// \param  direction           Unit vector from the listener to the point source, relative to the listener's
    ///                             coordinate system.
    /// \param  interpolation       The interpolation technique to use when rendering a point source at a location
    ///                             that is not contained in the measured HRTF data used by Phonon. **If using a custom
    ///                             HRTF database, this value must be set to IPL_HRTFINTERPOLATION_BILINEAR.**
    /// \param  outputAudio         Audio buffer that should contain the rendered audio data. The format of this
    ///                             buffer must match the \c outputFormat parameter passed to
    ///                             \c ::iplCreateBinauralEffect.
    ///
    pub fn iplApplyBinauralEffect(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        direction: IPLVector3,
        interpolation: IPLHrtfInterpolation,
        outputAudio: IPLAudioBuffer,
    );

    pub fn iplApplyBinauralEffectWithParameters(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        direction: IPLVector3,
        interpolation: IPLHrtfInterpolation,
        outputAudio: IPLAudioBuffer,
        leftDelay: *mut IPLfloat32,
        rightDelay: *mut IPLfloat32,
    );

    /** Resets any internal state maintained by an Object-Based Binaural Effect object. This is useful if the
     *  Object-Based Binaural Effect object is going to be disabled/unused for a few frames; resetting the internal
     *  state will prevent an audible glitch when the Object-Based Binaural Effect object is re-enabled at a later
     *  time.
     *
     *  \param  effect              Handle to an Object-Based Binaural Effect object.
     */
    pub fn iplFlushBinauralEffect(effect: IPLhandle);
}

///*****************************************************************************************************************/
///* Virtual Surround Effect                                                                                       */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Virtual Surround Effect object. This can be used to render a multichannel surround sound data using
    /// HRTF-based binaural rendering.
    ///
    /// \param  renderer            Handle to a Binaural Renderer object.
    /// \param  inputFormat         The format of the audio buffers that will be passed as input to this effect. All
    ///                             subsequent calls to \c ::iplApplyVirtualSurroundEffect for this effect object must
    ///                             use \c IPLAudioBuffer objects with the same format as specified here. The input
    ///                             format must not be Ambisonics.
    /// \param  outputFormat        The format of the audio buffers which will be used to retrieve the output from this
    ///                             effect. All subsequent calls to \c ::iplApplyVirtualSurroundEffect for this effect
    ///                             object must use \c IPLAudioBuffer objects with the same format as specified here.
    ///                             The output format must be stereo (2 channels).
    /// \param  effect              [out] Handle to the created Virtual Surround Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateVirtualSurroundEffect(
        renderer: IPLhandle,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Virtual Surround Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Virtual Surround Effect object to destroy.
    ///
    pub fn iplDestroyVirtualSurroundEffect(effect: *mut IPLhandle);

    /// Applies HRTF-based binaural rendering to a buffer of multichannel audio data.
    ///
    /// \param  effect              Handle to a Virtual Surround Effect.
    /// \param  inputAudio          Audio buffer containing the data to render using binaural rendering. The format of
    ///                             this buffer must match the \c inputFormat parameter passed to
    ///                             \c ::iplCreateVirtualSurroundEffect.
    /// \param  outputAudio         Audio buffer that should contain the rendered audio data. The format of this buffer
    ///                             must match the \c outputFormat parameter passed to
    ///                             \c ::iplCreateVirtualSurroundEffect.
    ///
    /// \remark When using a custom HRTF database, calling this function is not supported.
    ///
    pub fn iplApplyVirtualSurroundEffect(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        outputAudio: IPLAudioBuffer,
    );

    /// Resets any internal state maintained by a Virtual Surround Effect object. This is useful if the Virtual
    /// Surround Effect object is going to be disabled/unused for a few frames; resetting the internal state will
    /// prevent an audible glitch when the Virtual Surround Effect object is re-enabled at a later time.
    ///
    /// \param  effect              Handle to a Virtual Surround Effect object.
    ///
    pub fn iplFlushVirtualSurroundEffect(effect: IPLhandle);
}

///*****************************************************************************************************************/
///* Ambisonics Panning Effect                                                                                     */
///*****************************************************************************************************************/

extern "C" {
    /// Creates an Ambisonics Panning Effect object. This can be used to render higher-order Ambisonics data using
    /// standard panning algorithms.
    ///
    /// \param  renderer            Handle to a Binaural Renderer object.
    /// \param  inputFormat         The format of the audio buffers that will be passed as input to this effect. All
    ///                             subsequent calls to \c ::iplApplyAmbisonicsPanningEffect for this effect object must
    ///                             use \c IPLAudioBuffer objects with the same format as specified here. The input
    ///                             format must be Ambisonics.
    /// \param  outputFormat        The format of the audio buffers which will be used to retrieve the output from this
    ///                             effect. All subsequent calls to \c ::iplApplyAmbisonicsPanningEffect for this
    ///                             effect object must use \c IPLAudioBuffer objects with the same format as specified
    ///                             here.
    /// \param  effect              [out] Handle to the created Ambisonics Panning Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateAmbisonicsPanningEffect(
        renderer: IPLhandle,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys an Ambisonics Panning Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Ambisonics Panning Effect object to destroy.
    ///
    pub fn iplDestroyAmbisonicsPanningEffect(effect: *mut IPLhandle);

    /// Applies a panning-based rendering algorithm to a buffer of Ambisonics audio data. Ambisonics encoders and decoders
    /// use many different conventions to store the multiple Ambisonics channels, as well as different normalization
    /// schemes. Make sure that you correctly specify these settings when creating the Ambisonics Panning Effect
    /// object, otherwise the rendered audio will be incorrect.
    ///
    /// \param  effect              Handle to an Ambisonics Panning Effect object.
    /// \param  inputAudio          Audio buffer containing the data to render. The format of
    ///                             this buffer must match the \c inputFormat parameter passed to
    ///                             \c ::iplCreateAmbisonicsPanningEffect.
    /// \param  outputAudio         Audio buffer that should contain the rendered audio data. The format of this buffer
    ///                             must match the \c outputFormat parameter passed to
    ///                             \c ::iplCreateAmbisonicsPanningEffect.
    ///
    pub fn iplApplyAmbisonicsPanningEffect(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        outputAudio: IPLAudioBuffer,
    );

    /// Resets any internal state maintained by an Ambisonics Panning Effect object. This is useful if the Ambisonics
    /// Panning Effect object is going to be disabled/unused for a few frames; resetting the internal state will
    /// prevent an audible glitch when the Ambisonics Panning Effect object is re-enabled at a later time.
    ///
    /// \param  effect              Handle to an Ambisonics Panning Effect object.
    ///
    pub fn iplFlushAmbisonicsPanningEffect(effect: IPLhandle);

}

///*****************************************************************************************************************/
///* Ambisonics Binaural Effect                                                                                    */
///*****************************************************************************************************************/

extern "C" {
    /// Creates an Ambisonics Binaural Effect object. This can be used to render higher-order Ambisonics data using
    /// HRTF-based binaural rendering.
    ///
    /// \param  renderer            Handle to a Binaural Renderer object.
    /// \param  inputFormat         The format of the audio buffers that will be passed as input to this effect. All
    ///                             subsequent calls to \c ::iplApplyAmbisonicsBinauralEffect for this effect object must
    ///                             use \c IPLAudioBuffer objects with the same format as specified here. The input
    ///                             format must be Ambisonics.
    /// \param  outputFormat        The format of the audio buffers which will be used to retrieve the output from this
    ///                             effect. All subsequent calls to \c ::iplApplyAmbisonicsBinauralEffect for this
    ///                             effect object must use \c IPLAudioBuffer objects with the same format as specified
    ///                             here. The output format must be stereo (2 channels).
    /// \param  effect              [out] Handle to the created Ambisonics Binaural Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateAmbisonicsBinauralEffect(
        renderer: IPLhandle,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys an Ambisonics Binaural Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Ambisonics Binaural Effect object to destroy.
    ///
    pub fn iplDestroyAmbisonicsBinauralEffect(effect: *mut IPLhandle);

    /// Applies HRTF-based binaural rendering to a buffer of Ambisonics audio data. Ambisonics encoders and decoders
    /// use many different conventions to store the multiple Ambisonics channels, as well as different normalization
    /// schemes. Make sure that you correctly specify these settings when creating the Ambisonics Binaural Effect
    /// object, otherwise the rendered audio will be incorrect.
    ///
    /// \param  effect              Handle to an Ambisonics Binaural Effect object.
    /// \param  inputAudio          Audio buffer containing the data to render using binaural rendering. The format of
    ///                             this buffer must match the \c inputFormat parameter passed to
    ///                             \c ::iplCreateAmbisonicsBinauralEffect.
    /// \param  outputAudio         Audio buffer that should contain the rendered audio data. The format of this buffer
    ///                             must match the \c outputFormat parameter passed to
    ///                             \c ::iplCreateAmbisonicsBinauralEffect.
    ///
    /// \remark When using a custom HRTF database, calling this function is not supported.
    ///
    pub fn iplApplyAmbisonicsBinauralEffect(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        outputAudio: IPLAudioBuffer,
    );

    /// Resets any internal state maintained by an Ambisonics Binaural Effect object. This is useful if the Ambisonics
    /// Binaural Effect object is going to be disabled/unused for a few frames; resetting the internal state will
    /// prevent an audible glitch when the Ambisonics Binaural Effect object is re-enabled at a later time.
    ///
    /// \param  effect              Handle to an Ambisonics Binaural Effect object.
    ///
    pub fn iplFlushAmbisonicsBinauralEffect(effect: IPLhandle);
}

///*****************************************************************************************************************/
///* Environmental Renderer                                                                                        */
///*****************************************************************************************************************/

extern "C" {
    /// Creates an Environmental Renderer object.
    ///
    /// \param  context             The Context object used by the audio engine.
    /// \param  environment         Handle to an Environment object provided by the game engine. It is up to your
    ///                             application to pass this handle from the game engine to the audio engine.
    /// \param  renderingSettings   An \c IPLRenderingSettings object describing the audio pipeline's DSP processing
    ///                             parameters. These properties must remain constant throughout the lifetime of your
    ///                             application.
    /// \param  outputFormat        The audio format of the output buffers passed to any subsequent call to
    ///                             \c ::iplGetMixedEnvironmentalAudio. This format must not be changed once it is set
    ///                             during the call to this function.
    /// \param  threadCreateCallback    Pointer to a function that will be called when the internal simulation thread
    ///                                 is created. May be NULL.
    /// \param  threadDestroyCallback   Pointer to a function that will be called when the internal simulation thread
    ///                                 is destroyed. May be NULL.
    /// \param  renderer            [out] Handle to the created Environmental Renderer object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateEnvironmentalRenderer(
        context: IPLhandle,
        environment: IPLhandle,
        renderingSettings: IPLRenderingSettings,
        outputFormat: IPLAudioFormat,
        threadCreateCallback: IPLSimulationThreadCreateCallback,
        threadDestroyCallback: IPLSimulationThreadDestroyCallback,
        renderer: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys an Environmental Renderer object. If any other API objects are still referencing the Environmental
    /// Renderer object, the object will not be destroyed; it will only be destroyed once its reference count reaches
    /// zero.
    ///
    /// \param  renderer            [in, out] Address of a handle to the Environmental Renderer object to destroy.
    ///
    pub fn iplDestroyEnvironmentalRenderer(renderer: *mut IPLhandle);

    pub fn iplCreateSimulationData(
        simulationSettings: IPLSimulationSettings,
        renderingSettings: IPLRenderingSettings,
        simulationData: *mut IPLhandle,
    ) -> IPLerror;

    pub fn iplDestroySimulationData(simulationData: *mut IPLhandle);

    pub fn iplGetNumIrSamples(simulationData: IPLhandle) -> IPLint32;

    pub fn iplGetNumIrChannels(simulationData: IPLhandle) -> IPLint32;

    pub fn iplGenerateSimulationData(
        simulationData: IPLhandle,
        environment: IPLhandle,
        listenerPosition: IPLVector3,
        listenerAhead: IPLVector3,
        listenerUp: IPLVector3,
        sources: *mut IPLVector3,
    );

    pub fn iplGetSimulationResult(
        simulationData: IPLhandle,
        sourceIndex: IPLint32,
        channel: IPLint32,
        buffer: *mut IPLfloat32,
    );
}

///*****************************************************************************************************************/
///* Direct Sound                                                                                                  */
///*****************************************************************************************************************/

extern "C" {
    /// Calculates direct sound path parameters for a single source. It is up to the audio engine to perform audio
    /// processing that uses the information returned by this function.
    ///
    /// \param  environment         Handle to an Environment object.
    /// \param  listenerPosition    World-space position of the listener.
    /// \param  listenerAhead       Unit vector pointing in the direction in which the listener is looking.
    /// \param  listenerUp          Unit vector pointing upwards from the listener.
    /// \param  sourcePosition      World-space position of the source.
    /// \param  sourceRadius        Radius of the sphere defined around the source, for use with
    ///                             \c ::IPL_DIRECTOCCLUSION_VOLUMETRIC only.
    /// \param  occlusionMode       Confuguring the occlusion mode for direct path.
    /// \param  occlusionMethod     Algorithm to use for checking for direct path occlusion.
    ///
    /// \return Parameters of the direct path from the source to the listener.
    ///
    pub fn iplGetDirectSoundPath(
        environment: IPLhandle,
        listenerPosition: IPLVector3,
        listenerAhead: IPLVector3,
        listenerUp: IPLVector3,
        sourcePosition: IPLVector3,
        sourceRadius: IPLfloat32,
        occlusionMode: IPLDirectOcclusionMode,
        occlusionMethod: IPLDirectOcclusionMethod,
    ) -> IPLDirectSoundPath;
}

///*****************************************************************************************************************/
///* Direct Sound Effect                                                                                           */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Direct Sound Effect object.
    ///
    /// \param  renderer            Handle to an Environmental Renderer object.
    /// \param  inputFormat         The format of the audio buffers that will be passed as input to this effect. All
    ///                             subsequent calls to \c ::iplApplyDirectSoundEffect for this effect object must use
    ///                             \c IPLAudioBuffer objects with the same format as specified here.
    /// \param  outputFormat        The format of the audio buffers which will be used to retrieve the output from this
    ///                             effect. All subsequent calls to \c ::iplApplyDirectSoundEffect for this effect
    ///                             object must use \c IPLAudioBuffer objects with the same format as specified here.
    /// \param  effect              [out] Handle to the created Direct Sound Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateDirectSoundEffect(
        renderer: IPLhandle,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Direct Sound Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Direct Sound Effect object to destroy.
    ///
    pub fn iplDestroyDirectSoundEffect(effect: *mut IPLhandle);

    /// Applies various parameters in \c IPLDirectSoundPath to a buffer of audio data.
    ///
    /// \param  effect              Handle to a Direct Sound Effect object.
    /// \param  inputAudio          Audio buffer containing the dry audio data. The format of this buffer must match the
    ///                             \c inputFormat parameter passed to \c ::iplCreateDirectSoundEffect.
    /// \param  directSoundPath     Parameters of the direct path from the source to the listener.
    /// \param  options             Specifies which parameters from \c IPLDirectSoundPath should be processed by
    ///                             the Direct Sound Effect.
    /// \param  outputAudio         Audio buffer that should contain the wet audio data. The format of this buffer must
    ///                             match the \c outputFormat parameter passed to \c ::iplCreateDirectSoundEffect.
    ///
    pub fn iplApplyDirectSoundEffect(
        effect: IPLhandle,
        inputAudio: IPLAudioBuffer,
        directSoundPath: IPLDirectSoundPath,
        options: IPLDirectSoundEffectOptions,
        outputAudio: IPLAudioBuffer,
    );

    /// Resets any internal state maintained by a Direct Sound Effect object. This is useful if the
    /// Direct Sound Effect object is going to be disabled/unused for a few frames; resetting the internal
    /// state will prevent an audible glitch when the Direct Sound Effect object is re-enabled at a later
    /// time.
    ///
    /// \param  effect              Handle to a Direct Sound Effect object.
    ///
    pub fn iplFlushDirectSoundEffect(effect: IPLhandle);
}

///*****************************************************************************************************************/
///* Convolution Effect                                                                                            */
///*****************************************************************************************************************/

extern "C" {
    /// Creates a Convolution Effect object.
    ///
    /// \param  renderer            Handle to an Environmental Renderer object.
    /// \param  identifier          Unique identifier of the corresponding source, as defined in the baked data
    ///                             exported by the game engine. Each Convolution Effect object may have an identifier,
    ///                             which is used only if the Environment object provided by the game engine uses baked
    ///                             data for sound propagation. If so, the identifier of the Convolution Effect is used
    ///                             to look up the appropriate information from the baked data. Multiple Convolution
    ///                             Effect objects may be created with the same identifier; in that case they will use
    ///                             the same baked data.
    /// \param  simulationType      Whether this Convolution Effect object should use baked data or real-time simulation.
    /// \param  inputFormat         Format of all audio buffers passed as input to
    ///                             \c ::iplSetDryAudioForConvolutionEffect.
    /// \param  outputFormat        Format of all output audio buffers passed to \c ::iplGetWetAudioForConvolutionEffect.
    /// \param  effect              [out] Handle to the created Convolution Effect object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateConvolutionEffect(
        renderer: IPLhandle,
        identifier: IPLBakedDataIdentifier,
        simulationType: IPLSimulationType,
        inputFormat: IPLAudioFormat,
        outputFormat: IPLAudioFormat,
        effect: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Convolution Effect object.
    ///
    /// \param  effect              [in, out] Address of a handle to the Convolution Effect object to destroy.
    ///
    pub fn iplDestroyConvolutionEffect(effect: *mut IPLhandle);

    /// Changes the identifier associated with a Convolution Effect object. This is useful when using a static listener
    /// bake, where you may want to teleport the listener between two or more locations for which baked data has
    /// been generated.
    ///
    /// \param  effect              Handle to a Convolution Effect object.
    /// \param  identifier          The new identifier of the Convolution Effect object.
    ///
    pub fn iplSetConvolutionEffectIdentifier(effect: IPLhandle, identifier: IPLBakedDataIdentifier);

    /// Specifies a frame of dry audio for a Convolution Effect object. This is the audio data to which sound
    /// propagation effects should be applied.
    ///
    /// \param  effect              Handle to a Convolution Effect object.
    /// \param  sourcePosition      World-space position of the sound source emitting the dry audio.
    /// \param  dryAudio            Audio buffer containing the dry audio data.
    ///
    pub fn iplSetDryAudioForConvolutionEffect(
        effect: IPLhandle,
        sourcePosition: IPLVector3,
        dryAudio: IPLAudioBuffer,
    );

    /// Retrieves a frame of wet audio from a Convolution Effect object. This is the result of applying sound
    /// propagation effects to the dry audio previously specified using \c ::iplSetDryAudioForConvolutionEffect.
    ///
    /// \param  effect              Handle to a Convolution Effect object.
    /// \param  listenerPosition    World-space position of the listener.
    /// \param  listenerAhead       Unit vector in the direction in which the listener is looking.
    /// \param  listenerUp          Unit vector pointing upwards from the listener.
    /// \param  wetAudio            Audio buffer which will be populated with the wet audio data.
    ///
    pub fn iplGetWetAudioForConvolutionEffect(
        effect: IPLhandle,
        listenerPosition: IPLVector3,
        listenerAhead: IPLVector3,
        listenerUp: IPLVector3,
        wetAudio: IPLAudioBuffer,
    );

    /// Retrieves a mixed frame of wet audio. This is the sum of all wet audio data from all Convolution Effect
    /// objects that were created using the given Environmental Renderer object. Unless using TrueAudio Next for
    /// convolution, this is likely to provide a significant performance boost to the audio thread as compared to
    /// calling \c ::iplGetWetAudioForConvolutionEffect for each Convolution Effect separately. On the other hand, doing
    /// so makes it impossible to apply additional DSP effects for specific sources before mixing.
    ///
    /// \param  renderer            Handle to an Environmental Renderer object.
    /// \param  listenerPosition    World-space position of the listener.
    /// \param  listenerAhead       Unit vector in the direction in which the listener is looking.
    /// \param  listenerUp          Unit vector pointing upwards from the listener.
    /// \param  mixedWetAudio       Audio buffer which will be populated with the wet audio data.
    ///
    pub fn iplGetMixedEnvironmentalAudio(
        renderer: IPLhandle,
        listenerPosition: IPLVector3,
        listenerAhead: IPLVector3,
        listenerUp: IPLVector3,
        mixedWetAudio: IPLAudioBuffer,
    );

    /// Resets any internal state maintained by a Convolution Effect object. This is useful if the Convolution Effect
    /// object is going to be disabled/unused for a few frames; resetting the internal state will prevent an audible
    /// glitch when the Convolution Effect object is re-enabled at a later time.
    ///
    /// \param  effect              Handle to a Convolution Effect object.
    ///
    pub fn iplFlushConvolutionEffect(effect: IPLhandle);
}

///*****************************************************************************************************************/
///* Acoustic Probes                                                                                               */
///*****************************************************************************************************************/

extern "C" {
    /// Generates probes within a box. This function should typically be called from the game engine's editor, in
    /// response to the user indicating that they want to generate probes in the scene.
    ///
    /// \param  context                     Handle to the Context object used by the game engine.
    /// \param  scene                       Handle to the Scene object.
    /// \param  boxLocalToWorldTransform    4x4 local to world transform matrix laid out in column-major format.
    /// \param  placementParams             Parameters specifying how probes should be generated.
    /// \param  progressCallback            Pointer to a function that reports the percentage of this function's
    ///                                     work that has been completed. May be \c NULL.
    /// \param  probeBox                    [out] Handle to the created Probe Box object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateProbeBox(
        context: IPLhandle,
        scene: IPLhandle,
        boxLocalToWorldTransform: *mut IPLfloat32,
        placementParams: IPLProbePlacementParams,
        progressCallback: IPLProbePlacementProgressCallback,
        probeBox: *mut IPLhandle,
    ) -> IPLerror;

    /// Destroys a Probe Box object.
    ///
    /// \param  probeBox            [in, out] Address of a handle to the Probe Box object to destroy.
    ///
    pub fn iplDestroyProbeBox(probeBox: *mut IPLhandle);

    /// Retrieves spheres describing the positions and influence radii of all probes in the Probe Box object. This
    /// function should typically be called from the game engine's editor, and the retrieved spheres should be used
    /// for visualization.
    ///
    /// \param  probeBox            Handle to a Probe Box object.
    /// \param  probeSpheres        [out] Array into which information about the probe spheres is returned. It is the
    ///                             the caller's responsibility to manage memory for this array. The array must be
    ///                             large enough to hold all the spheres in the Probe Box object. May be \c NULL, in
    ///                             which case no spheres are returned; this is useful when finding out the number of
    ///                             probes in the Probe Box object.
    ///
    /// \return The number of probes in the Probe Box object.
    ///
    pub fn iplGetProbeSpheres(probeBox: IPLhandle, probeSpheres: *mut IPLSphere) -> IPLint32;

    /// Serializes a Probe Box object to a byte array. This is typically called by the game engine's editor in order
    /// to save the Probe Box object's data to disk.
    ///
    /// \param  probeBox            Handle to a Probe Box object.
    /// \param  data                [out] Byte array into which the Probe Box object will be serialized. It is the
    ///                             caller's responsibility to manage memory for this array. The array must be large
    ///                             enough to hold all the data in the Probe Box object. May be \c NULL, in which case
    ///                             no data is returned; this is useful when finding out the size of the data stored
    ///                             in the Probe Box object.
    ///
    /// \return Size (in bytes) of the serialized data.
    ///
    pub fn iplSaveProbeBox(probeBox: IPLhandle, data: *mut IPLbyte) -> IPLint32;

    /// Deserializes a Probe Box object from a byte array. This is typically called by the game engine's editor when
    /// loading a Probe Box object from disk.
    ///
    /// \param  context             Handle to the Context object used by the game engine.
    /// \param  data                Byte array containing the serialized representation of the Probe Box object. Must
    ///                             not be \c NULL.
    /// \param  size                Size (in bytes) of the serialized data.
    /// \param  probeBox            [out] Handle to the created Probe Box object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplLoadProbeBox(
        context: IPLhandle,
        data: *mut IPLbyte,
        size: IPLint32,
        probeBox: *mut IPLhandle,
    ) -> IPLerror;

    /// Creates a Probe Batch object. A Probe Batch object represents a set of probes that are loaded and unloaded
    /// from memory as a unit when the game is played. A Probe Batch may contain probes from multiple Probe Boxes;
    /// multiple Probe Batches may contain probes from the same Probe Box. At run-time, Phonon does not use Probe
    /// Boxes, it only needs Probe Batches. The typical workflow is as follows:
    ///
    /// 1.  Using the editor, the designer creates Probe Boxes to sample the scene.
    /// 2.  Using the editor, the designer specifies Probe Batches, and decides which probes are part of each Probe
    ///     Batch.
    /// 3.  The editor saves the Probe Batches along with the rest of the scene data for use at run-time.
    /// 4.  At run-time, Phonon uses the Probe Batches to retrieve baked data.
    ///
    /// \param  context             Handle to the Context object used by the game engine.
    /// \param  probeBatch          [out] Handle to the created Probe Batch object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateProbeBatch(context: IPLhandle, probeBatch: *mut IPLhandle) -> IPLerror;

    /// Destroys a Probe Batch object.
    ///
    /// \param  probeBatch          [in, out] Address of a handle to the Probe Batch object to destroy.
    ///
    pub fn iplDestroyProbeBatch(probeBatch: *mut IPLhandle);

    /// Adds a specific probe from a Probe Box to a Probe Batch. Once all probes in a Probe Box have been assigned to
    /// their respective Probe Batches, you can destroy the Probe Box object; the baked data for the probes will
    /// be retained by the Probe Batch.
    ///
    /// \param  probeBatch          Handle to a Probe Batch object into which the probe should be added.
    /// \param  probeBox            Handle to a Probe Box object from which the probe should be added.
    /// \param  probeIndex          Index of the probe to add. The index is defined relative to the array of probes
    ///                             returned by \c ::iplGetProbeSpheres.
    ///
    pub fn iplAddProbeToBatch(probeBatch: IPLhandle, probeBox: IPLhandle, probeIndex: IPLint32);

    /// Finalizes the set of probes that comprise a Probe Batch. Calling this function builds internal data
    /// structures that are used to rapidly determine which probes influence any given point in 3D space. You may
    /// not call \c ::iplAddProbeToBatch after calling this function. You must call this function before calling
    /// \c ::iplAddProbeBatch to add this Probe Batch object to a Probe Manager object.
    ///
    /// \param  probeBatch          Handle to a ProbeBatch object.
    ///
    pub fn iplFinalizeProbeBatch(probeBatch: IPLhandle);

    /// Serializes a Probe Batch object to a byte array. This is typically called by the game engine's editor in order
    /// to save the Probe Batch object's data to disk.
    ///
    /// \param  probeBatch          Handle to a Probe Batch object.
    /// \param  data                [out] Byte array into which the Probe Batch object will be serialized. It is the
    ///                             caller's responsibility to manage memory for this array. The array must be large
    ///                             enough to hold all the data in the Probe Batch object. May be \c NULL, in which
    ///                             case no data is returned; this is useful when finding out the size of the data
    ///                             stored in the Probe Batch object.
    ///
    /// \return Size (in bytes) of the serialized data.
    ///
    pub fn iplSaveProbeBatch(probeBatch: IPLhandle, data: *mut IPLbyte) -> IPLint32;

    /// Deserializes a Probe Batch object from a byte array. This is typically called by the game engine's editor when
    /// loading a Probe Batch object from disk. Calling this function implicitly calls \c ::iplFinalizeProbeBatch, so
    /// you do not need to call it explicitly.
    ///
    /// \param  context             Handle to the Context object used by the game engine.
    /// \param  data                Byte array containing the serialized representation of the Probe Batch object. Must
    ///                             not be \c NULL.
    /// \param  size                Size (in bytes) of the serialized data.
    /// \param  probeBatch          [out] Handle to the created Probe Batch object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplLoadProbeBatch(
        context: IPLhandle,
        data: *mut IPLbyte,
        size: IPLint32,
        probeBatch: *mut IPLhandle,
    ) -> IPLerror;

    /// Creates a Probe Manager object. A Probe Manager object manages a set of Probe Batch objects are runtime.
    /// It is typically exported from the game engine to the audio engine via an Environment object. Probe Batch
    /// objects can be dynamically added to or removed from a Probe Manager object.
    ///
    /// \param  context             Handle to the Context object used by the game engine.
    /// \param  probeManager        [out] Handle to the created Probe Manager object.
    ///
    /// \return Status code indicating whether or not the operation succeeded.
    ///
    pub fn iplCreateProbeManager(context: IPLhandle, probeManager: *mut IPLhandle) -> IPLerror;

    /// Destroys a Probe Manager object.
    ///
    /// \param  probeManager        [in, out] Address of a handle to the Probe Manager object to destroy.
    ///
    pub fn iplDestroyProbeManager(probeManager: *mut IPLhandle);

    /// Adds a Probe Batch to a Probe Manager object. Once this function returns, probes in the Probe Batch will be
    /// used to calculate sound propagation effects.
    ///
    /// \param  probeManager        Handle to a Probe Manager object.
    /// \param  probeBatch          Handle to the Probe Batch object to add.
    ///
    pub fn iplAddProbeBatch(probeManager: IPLhandle, probeBatch: IPLhandle);

    /// Removes a Probe Batch from a Probe Manager object. Once this function returns, probes in the Probe Batch will
    /// no longer be used to calculate sound propagation effects.
    ///
    /// \param  probeManager        Handle to a Probe Manager object.
    /// \param  probeBatch          Handle to the Probe Batch object to remove.
    ///
    pub fn iplRemoveProbeBatch(probeManager: IPLhandle, probeBatch: IPLhandle);
}

///*****************************************************************************************************************/
///* Baking                                                                                                        */
///*****************************************************************************************************************/

extern "C" {
    // Bakes reverb at all probes in a Probe Box. Phonon defines reverb as the indirect sound received at a probe
    // when a source is placed at the probe's location. This is a time-consuming operation, and should typically be
    // called from the game engine's editor.
    //
    // \param  environment         Handle to an Environment object.
    // \param  probeBox            Handle to the Probe Box containing the probes for which to bake reverb.
    // \param  bakingSettings      The kind of acoustic responses to bake.
    // \param  progressCallback    Pointer to a function that reports the percentage of this function's work that
    //                             has been completed. May be \c NULL.
    //
    //IPLAPI IPLvoid iplBakeReverb(IPLhandle environment, IPLhandle probeBox, IPLBakingSettings bakingSettings,
    //    IPLBakeProgressCallback progressCallback);

    // Bakes propagation effects from a specified source to all probes in a Probe Box. Sources are defined in terms
    // of a position and a sphere of influence; all probes in the Probe Box that lie within the sphere of influence
    // are processed by this function. This is a time-consuming operation, and should typically be called from the
    // game engine's editor.
    //
    // \param  environment         Handle to an Environment object.
    // \param  probeBox            Handle to the Probe Box containing the probes for which to bake reverb.
    // \param  sourceInfluence     Sphere defined by the source position (at its center) and its radius of
    //                             influence.
    // \param  sourceIdentifier    Identifier of the source. At run-time, a Convolution Effect object can use this
    //                             identifier to look up the correct impulse response information.
    // \param  bakingSettings      The kind of acoustic responses to bake.
    // \param  progressCallback    Pointer to a function that reports the percentage of this function's work that
    //                             has been completed. May be \c NULL.
    //
    //IPLAPI IPLvoid iplBakePropagation(IPLhandle environment, IPLhandle probeBox, IPLSphere sourceInfluence,
    //    IPLBakedDataIdentifier sourceIdentifier, IPLBakingSettings bakingSettings, IPLBakeProgressCallback progressCallback);

    // Bakes propagation effects from all probes in a Probe Box to a specified listener. Listeners are defined
    // solely by their position; their orientation may freely change at run-time. This is a time-consuming
    // operation, and should typically be called from the game engine's editor.
    //
    // \param  environment         Handle to an Environment object.
    // \param  probeBox            Handle to the Probe Box containing the probes for which to bake reverb.
    // \param  listenerInfluence   Position and influence radius of the listener.
    // \param  listenerIdentifier  Identifier of the listener. At run-time, a Convolution Effect object can use this
    //                             identifier to look up the correct impulse response information.
    // \param  bakingSettings      The kind of acoustic responses to bake.
    // \param  progressCallback    Pointer to a function that reports the percentage of this function's work that
    //                             has been completed. May be \c NULL.
    //
    //IPLAPI IPLvoid iplBakeStaticListener(IPLhandle environment, IPLhandle probeBox, IPLSphere listenerInfluence,
    //    IPLBakedDataIdentifier listenerIdentifier, IPLBakingSettings bakingSettings, IPLBakeProgressCallback progressCallback);

    /// Cancels any bake operations that may be in progress. Typically, an application will call \c ::iplBakeReverb
    /// or \c ::iplBakePropagation in a separate thread from the editor's GUI thread, to keep the GUI responsive.
    /// This function can be called from the GUI thread to safely and prematurely terminate execution of any
    /// of these functions.
    ///
    pub fn iplCancelBake();

    /// Deletes all baked data in a Probe Box that is associated with a given source. If no such baked data
    /// exists, this function does nothing.
    ///
    /// \param  probeBox            Handle to a Probe Box object.
    /// \param  identifier          Identifier of the source whose baked data is to be deleted.
    ///
    pub fn iplDeleteBakedDataByIdentifier(probeBox: IPLhandle, identifier: IPLBakedDataIdentifier);

    /// Returns the size (in bytes) of the baked data stored in a Probe Box corresponding to a given source.
    /// This is useful for displaying statistics in the editor's GUI.
    ///
    /// \param  probeBox            Handle to a Probe Box object.
    /// \param  identifier          Identifier of the source whose baked data size is to be returned.
    ///
    /// \return Size (in bytes) of the baked data stored in the Probe Box corresponding to the source.
    ///
    pub fn iplGetBakedDataSizeByIdentifier(
        probeBox: IPLhandle,
        identifier: IPLBakedDataIdentifier,
    ) -> IPLint32;
}
