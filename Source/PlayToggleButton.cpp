/*
  ==============================================================================

    PlayToggleButton.cpp
    Created: 8 Aug 2025 10:32:59am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "PlayToggleButton.h"

//==============================================================================
PlayToggleButton::PlayToggleButton()
{
    setClickingTogglesState(true);
    setToggleState(false, juce::dontSendNotification);
    onClick = [this]()
        {
            bool bPlay = getToggleState();
            DBG("Play toggled: " << (bPlay ? "ON" : "OFF"));

        };
}

PlayToggleButton::~PlayToggleButton()
{
}

void PlayToggleButton::paint (juce::Graphics& g)
{
    playLookAndFeel->drawToggleButton(g, *this, isMouseOver(), isMouseButtonDown());
}

void PlayToggleButton::resized()
{

}
void PlayToggleButton::setImages(juce::Image on, juce::Image off)
{
    if (playLookAndFeel != nullptr);
    {
        playLookAndFeel->playOnImage = on;
        playLookAndFeel->playOffImage = off;
    }
}