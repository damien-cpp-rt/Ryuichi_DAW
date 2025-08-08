/*
  ==============================================================================

    SoundFileUI.cpp
    Created: 6 Aug 2025 4:52:15pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SoundFileUI.h"

//==============================================================================
SoundFileUI::SoundFileUI()
{
    soundListBox.setModel(soundPanel.get());
    soundListBox.setRowHeight(24);
    addAndMakeVisible(soundListBox);
}

SoundFileUI::~SoundFileUI()
{
}

void SoundFileUI::paint (juce::Graphics& g)
{
    
}

void SoundFileUI::resized()
{
    soundListBox.setBounds(getLocalBounds());
}

void SoundFileUI::addItem(const juce::File& file)
{
    soundPanel->items.add(file);
    soundListBox.updateContent();
}

void SoundFileUI::mouseWheelMove(const juce::MouseEvent& event, const juce::MouseWheelDetails& wheel)
{
    if (soundPanel->items.size() > 24)
    {
        soundListBox.mouseWheelMove(event, wheel);
    }
}

