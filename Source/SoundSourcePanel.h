/*
  ==============================================================================

    SoundSourcePanel.h
    Created: 4 Aug 2025 2:55:49pm
    Author:  KGA

  ==============================================================================
*/
#define SOUNDDIRECTORY "C:/Ryuichi/Sound_Files"
#define VSTDIRECTORY "C:/Ryuichi/VST_Files"
#pragma once

#include <JuceHeader.h>
#include "SoundFile.h"
#include "VSTFile.h"
#include "SoundFileUI.h"
#include "VSTFileUI.h"

enum class PanelMode
{
    None,
    SoundMode,
    VSTMode
};
//==============================================================================
/*
*/
class SoundSourcePanel : public juce::Component
{
public:
    SoundSourcePanel();
    ~SoundSourcePanel() override;
    void paint(juce::Graphics&) override;
    void resized() override;
 
private:
    SoundFile soundFileButton;
    VSTFile vstFileButton;
    std::unique_ptr<SoundFileUI> soundFile = std::make_unique<SoundFileUI>();
    std::unique_ptr<VSTFileUI> vstFile = std::make_unique<VSTFileUI>();
    juce::File soundDirectory = { SOUNDDIRECTORY };
    juce::File vstDirectory = { VSTDIRECTORY };
    PanelMode currentMode = PanelMode::None;

    void loadSoundFilesFromDirectory(const juce::File& directory);
    void loadVSTFilesFromDirectory(const juce::File& directory);
    void PanelSetting(PanelMode currentMode);
   
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (SoundSourcePanel)
};
