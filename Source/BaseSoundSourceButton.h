/*
  ==============================================================================

    BaseSoundSourceButton.h
    Created: 5 Aug 2025 9:52:05am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class BaseSoundSourceButton  : public juce::DrawableButton
{
public:
    BaseSoundSourceButton();
    ~BaseSoundSourceButton() override;

    void paint (juce::Graphics&) override;
    void resized() override;

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (BaseSoundSourceButton)
};
