/*
  ==============================================================================

    BPM.cpp
    Created: 8 Aug 2025 10:58:03am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "BPM.h"

//==============================================================================
BPM::BPM()
{
    juce::File backgroundFile(BACKGROUND_DIR_PATH);
    if (backgroundFile.existsAsFile())
    {
        backGroundImage = juce::ImageFileFormat::loadFrom(backgroundFile);
    }
    bpmEditor.setText("60");
    bpmEditor.setJustification(juce::Justification::centred);
    bpmEditor.setInputRestrictions(0, "0123456789.");
    juce::Font Arial("Arial", 18.0f, juce::Font::plain);
    bpmEditor.setFont(Arial);
    bpmEditor.onTextChange = [this]()
        {
            auto text = bpmEditor.getText().getFloatValue();
            bpmValue = juce::jlimit(30.0f, 300.0f, text);
        };

    addAndMakeVisible(bpmEditor);
}

BPM::~BPM()
{
}

void BPM::paint (juce::Graphics& g)
{
    g.drawImage(backGroundImage, getLocalBounds().toFloat());
}

void BPM::resized()
{
    bpmEditor.setBounds(70, 5, 90, 30);
}
