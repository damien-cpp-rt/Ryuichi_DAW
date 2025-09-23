/*
  ==============================================================================

    VSTFile.h
    Created: 5 Aug 2025 9:54:36am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "BaseSoundSourceButton.h"
#include "AssetsPath.h"
//==============================================================================
/*
*/
class VSTFile  : public BaseSoundSourceButton
{
public:
    VSTFile();
    ~VSTFile() override;

    void paint (juce::Graphics&) override;
    void resized() override;

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (VSTFile)
};
