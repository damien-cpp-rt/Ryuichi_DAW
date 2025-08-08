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

class SubTrack  : public juce::Component,juce::DragAndDropTarget
{
public:
    SubTrack();
    ~SubTrack() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void itemDropped(const juce::DragAndDropTarget::SourceDetails& dragSourceDetails) override;
    struct soundBlockData
    {
        juce::File filePath;
        juce::String fileName;
        juce::Image soundWaveForm;
        bool isMuted = false;
        float pan = 0.0f;
        float soundBalance = 0.0f;
        bool hasReverb = false;
        bool hasDelay = false;
    };
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SubTrack)
    juce::Image subTrackBackGround;
    juce::Array<soundBlockData> soundFileArray;
    bool isInterestedInDragSource(const SourceDetails& dragSourceDetails) override;
};
