/*
  ==============================================================================

    SubTrackVolumeSlider.h
    Created: 6 Aug 2025 3:37:55pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class SubTrackVolumeSlider  : public juce::Slider
{
public:
    SubTrackVolumeSlider();
    ~SubTrackVolumeSlider() override;

    void paint (juce::Graphics&) override;
    void resized() override;

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (SubTrackVolumeSlider)
};
