/*
  ==============================================================================

    MuteToggleButton.cpp
    Created: 6 Aug 2025 1:53:12pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "MuteToggleButton.h"

//==============================================================================
MuteToggleButton::MuteToggleButton()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
    setClickingTogglesState(true);
    setToggleState(false, juce::dontSendNotification); 
    onClick = [this]()
        {
            bool bmuted = getToggleState();
            DBG("Mute toggled: " << (bmuted ? "ON" : "OFF"));
         
        };
}

MuteToggleButton::~MuteToggleButton()
{
}

void MuteToggleButton::paint (juce::Graphics& g)
{
   muteLookAndFeel->drawToggleButton(g, *this, isMouseOver(), isMouseButtonDown());
}

void MuteToggleButton::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..

}

void MuteToggleButton::setImages(juce::Image on, juce::Image off)
{
    if (muteLookAndFeel != nullptr);
    {
    muteLookAndFeel->muteOnImage = on;
    muteLookAndFeel->muteOffImage = off;
    }
}
