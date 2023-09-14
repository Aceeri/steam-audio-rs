
# Updating include headers

## Phonon headers
1. Download latest release of steam audio here: https://github.com/ValveSoftware/steam-audio/releases
2. Copy-paste files in the `include/` folder to here.

## Steam-Audio <-> FMOD headers
Currently not included in the release for whatever reason so just:
```
git clone https://github.com/ValveSoftware/steam-audio
cp steam-audio/fmod/src/steamaudio_fmod.h steam-audio-rs/steam-audio-sys/headers/
```
