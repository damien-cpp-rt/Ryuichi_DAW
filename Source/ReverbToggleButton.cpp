/*
  ==============================================================================

    ReverbToggleButton.cpp
    Created: 7 Aug 2025 4:51:08pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "ReverbToggleButton.h"

//==============================================================================
ReverbToggleButton::ReverbToggleButton()
{
    setClickingTogglesState(true);
    setToggleState(false, juce::dontSendNotification);
    onClick = [this]()
        {
            bool breverb = getToggleState();
        };
}

ReverbToggleButton::~ReverbToggleButton()
{
}

void ReverbToggleButton::paint (juce::Graphics& g)
{
    reverbLookAndFeel->drawToggleButton(g, *this, isMouseOver(), isMouseButtonDown());
}

void ReverbToggleButton::resized()
{

}
void ReverbToggleButton::setImages(juce::Image on, juce::Image off)
{
    if (reverbLookAndFeel != nullptr)
    {
        reverbLookAndFeel->reverbOnImage = on;
        reverbLookAndFeel->reverbOffImage = off;
    }
}
