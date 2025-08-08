/*
  ==============================================================================

    PlayLookAndFeel.cpp
    Created: 8 Aug 2025 10:33:39am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "PlayLookAndFeel.h"

//==============================================================================
PlayLookAndFeel::PlayLookAndFeel()
{

}

PlayLookAndFeel::~PlayLookAndFeel()
{
}

void PlayLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed)
{
    (void)isHighlighted;
    (void)isPressed;
    auto bounds = button.getLocalBounds().toFloat();
    auto img = button.getToggleState() ? playOnImage : playOffImage;
    g.drawImageWithin(img, 0, 0, static_cast<int>(bounds.getWidth()), static_cast<int>(bounds.getHeight()), juce::RectanglePlacement::centred);
}
