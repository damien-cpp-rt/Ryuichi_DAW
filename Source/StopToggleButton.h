/*
  ==============================================================================

    StopToggleButton.h
    Created: 8 Aug 2025 10:33:14am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "StopLookAndFeel.h"
//==============================================================================
/*
*/
class StopToggleButton  : public juce::ToggleButton
{
public:
    StopToggleButton();
    ~StopToggleButton() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void setImages(juce::Image on, juce::Image off);
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(StopToggleButton)
    std::unique_ptr<StopLookAndFeel>stopLookAndFeel = std::make_unique<StopLookAndFeel>();

};
