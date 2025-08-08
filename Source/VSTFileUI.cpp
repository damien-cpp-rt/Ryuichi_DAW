/*
  ==============================================================================

    VSTFileUI.cpp
    Created: 6 Aug 2025 4:52:28pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "VSTFileUI.h"

//==============================================================================
VSTFileUI::VSTFileUI()
{
    vstListBox.setModel(vstPanel.get());
    vstListBox.setRowHeight(24);
    addAndMakeVisible(vstListBox);
}

VSTFileUI::~VSTFileUI()
{
}

void VSTFileUI::paint (juce::Graphics& g)
{
   
}

void VSTFileUI::resized()
{
    vstListBox.setBounds(getLocalBounds());
}
void VSTFileUI::addItem(const juce::File& file)
{
    vstPanel->items.add(file);
    vstListBox.updateContent();
}
void VSTFileUI::mouseWheelMove(const juce::MouseEvent& event, const juce::MouseWheelDetails& wheel)
{
    if (vstPanel->items.size() > 24)
    {
        vstListBox.mouseWheelMove(event, wheel);
    }
}