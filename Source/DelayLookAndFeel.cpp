/*
  ==============================================================================

    DelayLookAndFeel.cpp
    Created: 7 Aug 2025 4:53:43pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "DelayLookAndFeel.h"

//==============================================================================
DelayLookAndFeel::DelayLookAndFeel()
{

}

DelayLookAndFeel::~DelayLookAndFeel()
{
}
void DelayLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed)
{
    (void)isHighlighted;
    (void)isPressed;
    auto bounds = button.getLocalBounds().toFloat();
    auto img = button.getToggleState() ? delayOnImage : delayOffImage;
    g.drawImageWithin(img, 0, 0, static_cast<int>(bounds.getWidth()), static_cast<int>(bounds.getHeight()), juce::RectanglePlacement::centred);
}