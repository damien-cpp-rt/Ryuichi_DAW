/*
  ==============================================================================

    StopLookAndFeel.h
    Created: 8 Aug 2025 10:33:55am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class StopLookAndFeel  : public juce::LookAndFeel_V4
{
public:
    StopLookAndFeel();
    ~StopLookAndFeel() override;
    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed) override;
    juce::Image stopOnImage, stopOffImage;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (StopLookAndFeel)
};
