/*
  ==============================================================================

    SubTrackController.h
    Created: 6 Aug 2025 1:12:09pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "MuteToggleButton.h"
#include "AssetsPath.h"

//==============================================================================
/*
*/
class SubTrackController  : public juce::Component
{
public:
    SubTrackController();
    ~SubTrackController() override;

    void paint (juce::Graphics&) override;
    void resized() override;

    MuteToggleButton muteToggleButton;
    juce::Slider slider;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SubTrackController)
    juce::Image subTrackContorllerBackGround;
};
