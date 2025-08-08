/*
  ==============================================================================

    MuteLookAndFeel.cpp
    Created: 6 Aug 2025 2:50:15pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "MuteLookAndFeel.h"

//==============================================================================
MuteLookAndFeel::MuteLookAndFeel()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.

}

MuteLookAndFeel::~MuteLookAndFeel()
{
}

void MuteLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button, bool isHighlighted, bool isPressed)
{
    (void)isHighlighted;
    (void)isPressed;
    auto bounds = button.getLocalBounds().toFloat();
    auto img = button.getToggleState() ? muteOnImage : muteOffImage;
    g.drawImageWithin(img, 0, 0, static_cast<int>(bounds.getWidth()), static_cast<int>(bounds.getHeight()), juce::RectanglePlacement::centred);
}