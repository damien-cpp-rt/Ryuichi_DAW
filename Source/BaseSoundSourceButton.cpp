/*
  ==============================================================================

    BaseSoundSourceButton.cpp
    Created: 5 Aug 2025 9:52:05am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "BaseSoundSourceButton.h"

//==============================================================================
BaseSoundSourceButton::BaseSoundSourceButton() : juce::DrawableButton("SoundSourceButton", juce::DrawableButton::ButtonStyle::ImageOnButtonBackground)
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.

}

BaseSoundSourceButton::~BaseSoundSourceButton()
{
}

void BaseSoundSourceButton::paint (juce::Graphics& g)
{
    /* This demo code just fills the component's background and
       draws some placeholder text to get you started.

       You should replace everything in this method with your own
       drawing code..
    */
}

void BaseSoundSourceButton::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..

}
