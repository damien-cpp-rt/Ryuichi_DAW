/*
  ==============================================================================

    SoundFile.h
    Created: 5 Aug 2025 9:53:56am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "BaseSoundSourceButton.h"

//==============================================================================
/*
*/
class SoundFile  : public BaseSoundSourceButton
{
public:
    SoundFile();
    ~SoundFile() override;

    void paint (juce::Graphics&) override;
    void resized() override;

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (SoundFile)
};
