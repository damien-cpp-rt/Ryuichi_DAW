/*
  ==============================================================================

    soundVectorData.h
    Created: 11 Aug 2025 1:07:54pm
    Author:  KGA

  ==============================================================================
*/

#pragma once
#include <JuceHeader.h>
namespace SoundCore
{
    struct soundVecterData
    {
        int trackNumber = -1;
        juce::Array<juce::File> filePaths;
        juce::Array<juce::String> fileNames;
        juce::Array<juce::Image> soundWaveForm;
        float volume = 0.5f;
        bool isMuted = false;
        float pan = 0.0f;
        float soundBalance = 0.0f;
        bool hasReverb = false;
        bool hasDelay = false;
    };
}
