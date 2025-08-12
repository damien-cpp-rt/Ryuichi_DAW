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
    std::function<void(const juce::File& path, const juce::String& name)> onFileDrepped;
    void paint (juce::Graphics&) override;
    void resized() override;
    void itemDropped(const juce::DragAndDropTarget::SourceDetails& dragSourceDetails) override;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SubTrack)
    juce::Image subTrackBackGround;
    bool isInterestedInDragSource(const SourceDetails& dragSourceDetails) override;
};
