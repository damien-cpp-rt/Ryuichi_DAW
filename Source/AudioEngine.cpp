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

void AudioEngine::rust_string_delete(char* s)
{
    rust_free_string(s);
}

//void AudioEngine::rust_start_sound(bool bstart)
//{
//    if (bstart)
//    {
//        if (rust_sound_play(eng.get())) { DBG("[Rust_Sound_Play] Ok");}
//        else { DBG("[Rust_Sound_Play] ERROR"); }
//    }
//    else 
//    {
//        if (rust_sound_stop(eng.get())) { DBG("[Rust_Sound_Stop] Ok"); }
//        else { { DBG("[Rust_Sound_Stop] ERROR"); } }
//    }
//}


bool AudioEngine::rust_file_update(int32_t number, const char* path, uint64_t tl_start, uint64_t tl_len, uint32_t src)
{
    return rust_sound_add_clip(eng.get(), number, path, tl_start, tl_len, src);
}

bool AudioEngine::rust_file_move(int32_t old_track, uint64_t old_start, int32_t new_track, uint64_t new_start)
{
    return rust_sound_move_clip_by_start(eng.get(), old_track, old_start, new_track, new_start);
}

bool AudioEngine::rust_file_delet(int32_t track, uint64_t start)
{
    return rust_sound_delete_clip_by_start(eng.get(), track, start);
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
