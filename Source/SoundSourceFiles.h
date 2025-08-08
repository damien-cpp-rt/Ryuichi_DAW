/*
  ==============================================================================

    SoundSourceFiles.h
    Created: 4 Aug 2025 2:18:53pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "SoundSourcePanel.h"
#include "CloseButtonBrowser.h"


//==============================================================================
/*
*/
class SoundSourceFiles  : public juce::Component
{
public:
    SoundSourceFiles();
    ~SoundSourceFiles() override;

    void paint (juce::Graphics&) override;
    void resized() override;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SoundSourceFiles)
    juce::ImageComponent windowBarComponent;
    SoundSourcePanel sourcePanel;
    std::unique_ptr<CloseButtonBrowser> soundFilesCloseButton = std::make_unique<CloseButtonBrowser>();
};
