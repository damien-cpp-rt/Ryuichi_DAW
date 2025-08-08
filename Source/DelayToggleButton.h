/*
  ==============================================================================

    DelayToggleButton.h
    Created: 7 Aug 2025 4:50:50pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "DelayLookAndFeel.h"

//==============================================================================
/*
*/
class DelayToggleButton  : public juce::ToggleButton
{
public:
    DelayToggleButton();
    ~DelayToggleButton() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void setImages(juce::Image on, juce::Image off);
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(DelayToggleButton)
    std::unique_ptr<DelayLookAndFeel> delayLookAndFeel = std::make_unique<DelayLookAndFeel>();
};
