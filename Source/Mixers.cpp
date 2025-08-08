/*
  ==============================================================================

    Mixers.cpp
    Created: 7 Aug 2025 3:24:14pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "Mixers.h"

//==============================================================================
Mixers::Mixers()
{
#pragma region WindowBar,CloseButton
    juce::File windowBarFile(MIXERWINDOWBARDIRPATH);
    if (windowBarFile.existsAsFile())
    {
        juce::Image windowImg = juce::ImageFileFormat::loadFrom(windowBarFile);
        mixerWindowBar.setImage(windowImg);
        addAndMakeVisible(&mixerWindowBar);
    }
    if (mixerCloseButton != nullptr)
    {
    addAndMakeVisible(mixerCloseButton.get());
    mixerCloseButton->onClick = [this]()
        {
            setVisible(false);
        };
    }
#pragma endregion
    if (trackMixer_0 != nullptr)
    {
        addAndMakeVisible(trackMixer_0.get());
    }
    if (trackMixer_1 != nullptr)
    {
        addAndMakeVisible(trackMixer_1.get());
    }
    if (trackMixer_2 != nullptr)
    {
        addAndMakeVisible(trackMixer_2.get());
    }
    if (trackMixer_3 != nullptr)
    {
        addAndMakeVisible(trackMixer_3.get());
    }
}


Mixers::~Mixers()
{

}

void Mixers::paint (juce::Graphics& g)
{
  
}

void Mixers::resized()
{
    mixerWindowBar.setBounds(0, 0, 1200, 40);
    mixerCloseButton->setBounds(1160, 5, 30, 30);
    trackMixer_0->setBounds(0, 40, 300, 200);
    trackMixer_1->setBounds(300, 40, 300, 200);
    trackMixer_2->setBounds(600, 40, 300, 200);
    trackMixer_3->setBounds(900, 40, 300, 200);
}
