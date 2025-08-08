/*
  ==============================================================================

    PlayToggleButton.h
    Created: 8 Aug 2025 10:32:59am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "PlayLookAndFeel.h"
//==============================================================================
/*
*/
class PlayToggleButton  : public juce::ToggleButton
{
public:
    PlayToggleButton();
    ~PlayToggleButton() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void setImages(juce::Image on, juce::Image off);
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(PlayToggleButton)
    std::unique_ptr<PlayLookAndFeel>playLookAndFeel = std::make_unique<PlayLookAndFeel>();
};
