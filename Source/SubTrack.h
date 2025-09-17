/*
  ==============================================================================

    SubTrack.h
    Created: 6 Aug 2025 4:30:42pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "ClipData.h"
#include "TimeLineState.h"

class SubTrack  : public juce::Component 
{
public:
    SubTrack();
    ~SubTrack() override;
    void paint (juce::Graphics&) override;
    void resized() override;
    juce::Array<juce::Image>* soundTrackImg =nullptr;
#pragma region FileDrep
    void bindTimeline(TimeLine::timeLineState* tl) { timeline = tl; }
    void bindClips(juce::OwnedArray<ClipData>* list) { clips = list; }
    float getLastClickX() const { return lastClickX; }
    void mouseDown(const juce::MouseEvent& e) override { lastClickX = e.position.x; }
#pragma endregion
    uint64_t* playheadSamples = nullptr;
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SubTrack)
#pragma region FileDrep
    void drawBeatGrid(juce::Graphics& g, juce::Rectangle<int> area);

    TimeLine::timeLineState* timeline = nullptr;
    juce::OwnedArray<ClipData>* clips = nullptr;
    float lastClickX = 0.0f;
#pragma endregion
    juce::Image subTrackBackGround;
};
