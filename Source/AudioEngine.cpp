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
