/*
  ==============================================================================

    AudioEngine.h
    Created: 11 Aug 2025 11:50:37am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "soundVectorData.h"
extern "C"
{
    const char* rust_sound_transform(const char* path, const char* name);
    void rust_free_string(char* s);
}
class MainComponent;
//==============================================================================
/*
*/
class AudioEngine  : public juce::Component
{
public:
    AudioEngine();
    ~AudioEngine() override;
    void paint (juce::Graphics&) override;
    void resized() override;
    const char* rust_waveform_create(const char* path, const char* name);
    void rust_string_delete(char* s);
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_0;
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_1;
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_2;
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_3;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (AudioEngine)
};
