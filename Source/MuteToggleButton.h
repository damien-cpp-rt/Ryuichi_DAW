/*
  ==============================================================================

    MuteToggleButton.h
    Created: 6 Aug 2025 1:53:12pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "MuteLookAndFeel.h"

//==============================================================================
/*
*/
class MuteToggleButton : public juce::ToggleButton
{
public:
    MuteToggleButton();
    ~MuteToggleButton() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void setImages(juce::Image on, juce::Image off);
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (MuteToggleButton)
    std::unique_ptr<MuteLookAndFeel> muteLookAndFeel = std::make_unique<MuteLookAndFeel>();
};
