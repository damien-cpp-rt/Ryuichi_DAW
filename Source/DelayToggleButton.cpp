/*
  ==============================================================================

    DelayToggleButton.cpp
    Created: 7 Aug 2025 4:50:50pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "DelayToggleButton.h"

//==============================================================================
DelayToggleButton::DelayToggleButton()
{
    setClickingTogglesState(true);
    setToggleState(false, juce::dontSendNotification);
    onClick = [this]()
        {
            bool breverb = getToggleState();
        };
}

DelayToggleButton::~DelayToggleButton()
{
}

void DelayToggleButton::paint (juce::Graphics& g)
{
    delayLookAndFeel->drawToggleButton(g, *this, isMouseOver(), isMouseButtonDown());
}

void DelayToggleButton::resized()
{

}

void DelayToggleButton::setImages(juce::Image on, juce::Image off)
{
    if (delayLookAndFeel != nullptr)
    {
        delayLookAndFeel->delayOnImage = on;
        delayLookAndFeel->delayOffImage = off;
    }
}