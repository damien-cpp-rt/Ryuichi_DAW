/*
  ==============================================================================

    MuteLookAndFeel.h
    Created: 6 Aug 2025 2:50:15pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class MuteLookAndFeel  : public juce::LookAndFeel_V4
{
public:
    MuteLookAndFeel();
    ~MuteLookAndFeel() override;
    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed) override;
    juce::Image muteOnImage, muteOffImage;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (MuteLookAndFeel)
};
