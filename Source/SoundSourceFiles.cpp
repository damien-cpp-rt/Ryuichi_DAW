/*
  ==============================================================================

    SoundSourceFiles.cpp
    Created: 4 Aug 2025 2:18:53pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SoundSourceFiles.h"

//==============================================================================
SoundSourceFiles::SoundSourceFiles()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
#pragma region WindowBar
    juce::File ImgFile("C:/Ryuichi/UI_Image/WindowBar.png");
    if (ImgFile.existsAsFile())
    {
        juce::Image img = juce::ImageFileFormat::loadFrom(ImgFile);
        windowBarComponent.setImage(img);
        addAndMakeVisible(&windowBarComponent);
    }
#pragma endregion
#pragma region SoundPanel
    addAndMakeVisible(sourcePanel);
#pragma endregion
#pragma region CloseButton
    if (soundFilesCloseButton != nullptr)
    {
        addAndMakeVisible(soundFilesCloseButton.get());
        setVisible(true);
    soundFilesCloseButton->onClick = [this]()
        {
            DBG("SoundSourceFilesExit");
            setVisible(false);
        };
    }
#pragma endregion
}

SoundSourceFiles::~SoundSourceFiles()
{
}

void SoundSourceFiles::paint (juce::Graphics& g)
{
    g.fillAll (juce::Colour::fromString("#383838"));
    g.setColour (juce::Colour::fromString("#444444"));
    g.drawRect (getLocalBounds(), 0.5);
}

void SoundSourceFiles::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..
    windowBarComponent.setBounds(0, 0, 300,40);
    sourcePanel.setBounds(0, 40, 300, 1050);
    soundFilesCloseButton->setBounds(260, 5, 30, 30);
}
