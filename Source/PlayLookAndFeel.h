/*
  ==============================================================================

    PlayLookAndFeel.h
    Created: 8 Aug 2025 10:33:39am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class PlayLookAndFeel  : public juce::LookAndFeel_V4
{
public:
    PlayLookAndFeel();
    ~PlayLookAndFeel() override;
    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed) override;
    juce::Image playOnImage, playOffImage;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (PlayLookAndFeel)
};
