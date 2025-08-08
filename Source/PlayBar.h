/*
  ==============================================================================

    PlayBar.h
    Created: 8 Aug 2025 10:23:44am
    Author:  KGA

  ==============================================================================
*/

#pragma once
#define TITLE_DIR_PATH "C:/Ryuichi/UI_Image/PlayBar.png"
#define PLAY_ON_DIR_PATH "C:/Ryuichi/UI_Image/play_Button_on.png"
#define PLAY_OFF_DIR_PATH "C:/Ryuichi/UI_Image/play_Button_off.png"
#define STOP_ON_DIR_PATH "C:/Ryuichi/UI_Image/Stop_Button_on.png"
#define STOP_OFF_DIR_PATH "C:/Ryuichi/UI_Image/Stop_Button_off.png"
#define BPMTEXT_DIR_PATH "C:/Ryuichi/UI_Image/BPMText.png"

#include <JuceHeader.h>
#include "PlayToggleButton.h"
#include "StopToggleButton.h"
#include "BPM.h"
//==============================================================================
/*
*/
class PlayBar  : public juce::Component
{
public:
    PlayBar();
    ~PlayBar() override;

    void paint (juce::Graphics&) override;
    void resized() override;

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(PlayBar)
    juce::ImageComponent titleImage;
    juce::ImageComponent bpmTextImage;
    PlayToggleButton playToggleButton;
    StopToggleButton stopToggleButton;
    BPM bpm;
};
