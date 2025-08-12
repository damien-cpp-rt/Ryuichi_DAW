/*
  ==============================================================================

    SubTrack.h
    Created: 6 Aug 2025 4:30:42pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/

class SubTrack  : public juce::Component
{
public:
    SubTrack();
    ~SubTrack() override;
    std::function<void(const juce::File& path, const juce::String& name)> onFileDrepped;
    void paint (juce::Graphics&) override;
    void resized() override;
    void mainTrackFileTransmission(const juce::String filePath);
    juce::Array<juce::Image>* soundTrackImg =nullptr;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SubTrack)
    juce::Image subTrackBackGround;
};
