/*
  ==============================================================================

    Mixers.h
    Created: 7 Aug 2025 3:24:14pm
    Author:  KGA

  ==============================================================================
*/
#define MIXERWINDOWBARDIRPATH "C:/Ryuichi/UI_Image/TrackBar.png"
#pragma once

#include <JuceHeader.h>
#include "CloseButtonBrowser.h"
#include "TrackMixers.h"

//==============================================================================
/*
*/
class Mixers  : public juce::Component
{
public:
    Mixers();
    ~Mixers() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    std::unique_ptr<TrackMixers> trackMixer_0 = std::make_unique<TrackMixers>();
    std::unique_ptr<TrackMixers> trackMixer_1 = std::make_unique<TrackMixers>();
    std::unique_ptr<TrackMixers> trackMixer_2 = std::make_unique<TrackMixers>();
    std::unique_ptr<TrackMixers> trackMixer_3 = std::make_unique<TrackMixers>();
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(Mixers)
    juce::ImageComponent mixerWindowBar;
    std::unique_ptr<CloseButtonBrowser> mixerCloseButton = std::make_unique<CloseButtonBrowser>();
};
