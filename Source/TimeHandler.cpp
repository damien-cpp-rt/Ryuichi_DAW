/*
  ==============================================================================

    TimeHandler.cpp
    Created: 8 Sep 2025 9:43:53am
    Author:  KGA

  ==============================================================================
*/

#include "TimeHandler.h"
#include "AudioEngine.h"

TimeHandler::TimeHandler(AudioEngine& aeng, juce::Slider& playhead, TimeLine::timeLineState& tl, bool& isplay, bool& userDragging) : playhead(playhead), aEng(aeng), timeline(tl), isPlaying(isplay) , userDragging(userDragging)
{
    startTimerHz(60);
}

TimeHandler::~TimeHandler()
{
}

void TimeHandler::timerCallback()
{
    if (userDragging) return;

    const auto sr = aEng.rust_get_sr();
    const auto pos = aEng.rust_get_pos();
    isPlaying = aEng.rust_get_is_playing();
    
    timeline.sr = sr;
    playhead.setValue((double)pos, juce::dontSendNotification);
}
