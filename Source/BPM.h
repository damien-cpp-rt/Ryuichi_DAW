/*
  ==============================================================================

    BPM.h
    Created: 8 Aug 2025 10:58:03am
    Author:  KGA

  ==============================================================================
*/

#pragma once
#define BACKGROUND_DIR_PATH "C:/Ryuichi/UI_Image/BPMBox.png"
#include <JuceHeader.h>

//==============================================================================
/*
*/
class BPM  : public juce::Component
{
public:
    BPM();
    ~BPM() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    float bpmValue = 120.0f;
private:
    juce::Image backGroundImage;
    juce::TextEditor bpmEditor;
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (BPM)
};
