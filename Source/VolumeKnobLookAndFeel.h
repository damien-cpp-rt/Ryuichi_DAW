/*
  ==============================================================================

    VolumeKnobLookAndFeel.h
    Created: 8 Aug 2025 2:08:14pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class VolumeKnobLookAndFeel  : public juce::LookAndFeel_V4
{
public:
    VolumeKnobLookAndFeel();
    ~VolumeKnobLookAndFeel() override;
    void drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
        float sliderPosProportional, float rotaryStartAngle,
        float rotaryEndAngle, juce::Slider& slider) override;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (VolumeKnobLookAndFeel)
};
