/*
  ==============================================================================

    ReverbLookAndFeel.h
    Created: 7 Aug 2025 4:54:04pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class ReverbLookAndFeel  : public juce::LookAndFeel_V4
{
public:
    ReverbLookAndFeel();
    ~ReverbLookAndFeel() override;
    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed) override;
    juce::Image reverbOnImage, reverbOffImage;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ReverbLookAndFeel)
};
