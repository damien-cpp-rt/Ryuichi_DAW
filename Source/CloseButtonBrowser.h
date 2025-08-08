/*
  ==============================================================================

    CloseButtonBrowser.h
    Created: 4 Aug 2025 3:13:52pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class CloseButtonBrowser  : public juce::DrawableButton
{
public:
    CloseButtonBrowser();
    ~CloseButtonBrowser() override;

    void paint (juce::Graphics&) override;
    void resized() override;

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (CloseButtonBrowser)
};
