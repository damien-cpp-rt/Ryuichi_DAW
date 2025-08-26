/*
  ==============================================================================

    AudioEngine.h
    Created: 11 Aug 2025 11:50:37am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include <vector>
#include "soundVectorData.h"
extern "C"
{
    struct Engine;
    Engine* rust_audio_engine_new(std::int32_t number);
    void rust_audio_engine_free(Engine* engine);
    const char* rust_sound_transform(const char* path, const char* name);
    void rust_free_string(char* s); 
    bool rust_sound_file_update(Engine* engine, const char* path);
}
struct EngineDeleter {
    void operator()(Engine* e) const noexcept {
        if (e) rust_audio_engine_free(e);
    }
};
using EnginePtr = std::unique_ptr<Engine, EngineDeleter>;

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
    void rust_start_sound(bool bstart);
    bool rust_file_update(int tracknum, const char* path);
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_0;
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_1;
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_2;
    std::shared_ptr<SoundCore::soundVecterData> audioTrack_3;
    std::vector<EnginePtr> eng;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(AudioEngine)
};
