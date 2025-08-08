/*
  ==============================================================================

    ReverbLookAndFeel.cpp
    Created: 7 Aug 2025 4:54:04pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "ReverbLookAndFeel.h"

//==============================================================================
ReverbLookAndFeel::ReverbLookAndFeel()
{

}

ReverbLookAndFeel::~ReverbLookAndFeel()
{
}

void ReverbLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed)
{
    (void)isHighlighted;
    (void)isPressed;
    auto bounds = button.getLocalBounds().toFloat();
    auto img = button.getToggleState() ? reverbOnImage : reverbOffImage;
    g.drawImageWithin(img, 0, 0, static_cast<int>(bounds.getWidth()), static_cast<int>(bounds.getHeight()), juce::RectanglePlacement::centred);
}
