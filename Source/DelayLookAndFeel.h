/*
  ==============================================================================

    DelayLookAndFeel.h
    Created: 7 Aug 2025 4:53:43pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>


//==============================================================================
/*
*/
class DelayLookAndFeel  : public juce::LookAndFeel_V4
{
public:
    DelayLookAndFeel();
    ~DelayLookAndFeel() override;
    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed) override;
    juce::Image delayOnImage, delayOffImage;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (DelayLookAndFeel)
};
