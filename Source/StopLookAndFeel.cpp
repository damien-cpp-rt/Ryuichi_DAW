/*
  ==============================================================================

    StopLookAndFeel.cpp
    Created: 8 Aug 2025 10:33:55am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "StopLookAndFeel.h"

//==============================================================================
StopLookAndFeel::StopLookAndFeel()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.

}

StopLookAndFeel::~StopLookAndFeel()
{
}
void StopLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed)
{
    (void)isHighlighted;
    (void)isPressed;
    auto bounds = button.getLocalBounds().toFloat();
    auto img = button.getToggleState() ? stopOnImage : stopOffImage;
    g.drawImageWithin(img, 0, 0, static_cast<int>(bounds.getWidth()), static_cast<int>(bounds.getHeight()), juce::RectanglePlacement::centred);
}