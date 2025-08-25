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
    //engine new 0,1,2,3 Vector Save hipMomory Point
    eng.emplace_back(rust_audio_engine_new(0));
    eng.emplace_back(rust_audio_engine_new(1));
    eng.emplace_back(rust_audio_engine_new(2));
    eng.emplace_back(rust_audio_engine_new(3));
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

    }
    else 
    {

    }
}
