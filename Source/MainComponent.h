#pragma once

#include <JuceHeader.h>
#include "SoundSourceFiles.h"
#include "MainTrack.h"
#include "Mixers.h"
#include "PlayBar.h"

//==============================================================================
/*
    This component lives inside our window, and this is where you should put all
    your controls and content.
*/
class MainComponent  : public juce::AnimatedAppComponent
{
public:
    //==============================================================================
    MainComponent();
    ~MainComponent() override;

    //==============================================================================
    void paint (juce::Graphics&) override;
    void resized() override;
    void update() override;

private:
    //==============================================================================
    // Your private member variables go here...
    juce::String backGroundName = "Ryuichi";
    SoundSourceFiles soundBrowser;
    MainTrack mainTrack;
    Mixers mixers;
    PlayBar playBar;
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (MainComponent)
};
