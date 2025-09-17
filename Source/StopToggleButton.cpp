/*
  ==============================================================================

    StopToggleButton.cpp
    Created: 8 Aug 2025 10:33:14am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "StopToggleButton.h"

//==============================================================================
StopToggleButton::StopToggleButton()
{
    setClickingTogglesState(false);
    setToggleState(true, juce::dontSendNotification);
    /*onClick = [this]()
        {
            bool bStop = getToggleState();
            DBG("Stop toggled: " << (bStop ? "ON" : "OFF"));

        };*/
}

StopToggleButton::~StopToggleButton()
{
}

void StopToggleButton::paint (juce::Graphics& g)
{
    stopLookAndFeel->drawToggleButton(g, *this, isMouseOver(), isMouseButtonDown());
}

void StopToggleButton::resized()
{

}
void StopToggleButton::setImages(juce::Image on, juce::Image off)
{
    if (stopLookAndFeel != nullptr)
    {
        stopLookAndFeel->stopOnImage = on;
        stopLookAndFeel->stopOffImage = off;
    }
    //이미지 전달
}