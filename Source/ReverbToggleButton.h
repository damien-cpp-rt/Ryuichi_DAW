/*
  ==============================================================================

    ReverbToggleButton.h
    Created: 7 Aug 2025 4:51:08pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "ReverbLookAndFeel.h"

//==============================================================================
/*
*/
class ReverbToggleButton  : public juce::ToggleButton
{
public:
    ReverbToggleButton();
    ~ReverbToggleButton() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void setImages(juce::Image on, juce::Image off);
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ReverbToggleButton)
    std::unique_ptr<ReverbLookAndFeel> reverbLookAndFeel = std::make_unique<ReverbLookAndFeel>();
};
