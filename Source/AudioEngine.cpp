/*
  ==============================================================================

    AudioEngine.cpp
    Created: 11 Aug 2025 11:50:37am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "AudioEngine.h"
#include "MainComponent.h"

//==============================================================================
AudioEngine::AudioEngine()
{
    TrackDatas* rust_track_0 =rust_audio_track_new(0);
    TrackDatas* rust_track_1 = rust_audio_track_new(1);
    TrackDatas* rust_track_2 = rust_audio_track_new(2);
    TrackDatas* rust_track_3 = rust_audio_track_new(3);
    Engine* raw = rust_audio_engine_new(rust_track_0, rust_track_1, rust_track_2, rust_track_3);
    if (!raw) {
        rust_audio_track_free(rust_track_0);
        rust_audio_track_free(rust_track_1);
        rust_audio_track_free(rust_track_2);
        rust_audio_track_free(rust_track_3);
        return;
    }
    eng.reset(raw);
}

AudioEngine::~AudioEngine()
{

}

void AudioEngine::paint (juce::Graphics& g)
{

}

void AudioEngine::resized()
{

}

const char* AudioEngine::rust_waveform_create(const char* path, const char* name)
{
    return rust_sound_transform(path, name);
}

void AudioEngine::rust_string_delete(char* s)
{
    rust_free_string(s);
}

void AudioEngine::rust_start_sound(bool bstart)
{
    if (bstart)
    {
        if (rust_sound_play(eng.get())) { DBG("[Rust_Sound_Play] Ok");}
        else { DBG("[Rust_Sound_Play] ERROR"); }
    }
    else 
    {
        if (rust_sound_stop(eng.get())) { DBG("[Rust_Sound_Stop] Ok"); }
        else { { DBG("[Rust_Sound_Stop] ERROR"); } }
    }
}

bool AudioEngine::rust_file_update(int tracknum,const char* path)
{
    if (!path) { return false; }
    if (tracknum < 0 || tracknum >= 4) { return false; }
    return rust_sound_file_update(eng.get(), path, tracknum);
}

bool AudioEngine::rust_volume_update(float volume, int tracknum)
{
    if (tracknum < 0 || tracknum >= 4) { return false; }
    return rust_sound_volume_update(eng.get(), volume, tracknum);
}

bool AudioEngine::rust_mute_update(bool muted, int tracknum)
{
    if (tracknum < 0 || tracknum >= 4) { return false; }
    return rust_sound_mute_update(eng.get(), muted, tracknum);
}

bool AudioEngine::rust_pan_update(float pan, int tracknum)
{
    if (tracknum < 0 || tracknum >= 4) { return false; }
    return rust_sound_pan_update(eng.get(), pan, tracknum);
}

bool AudioEngine::rust_bpm_update(float bpm)
{
    return rust_sound_bpm_update(eng.get(),bpm);
}

bool AudioEngine::rust_file_all_delete(int number)
{
    return rust_sound_file_all_delete(eng.get(), number);
}
