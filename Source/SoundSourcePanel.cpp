/*
  ==============================================================================

    SoundSourcePanel.cpp
    Created: 4 Aug 2025 2:55:49pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SoundSourcePanel.h"

//==============================================================================
SoundSourcePanel::SoundSourcePanel()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
#pragma region Button
    addAndMakeVisible(soundFileButton);
    addAndMakeVisible(vstFileButton);
#pragma endregion
#pragma region File
    if (soundFile != nullptr)
    {
        loadSoundFilesFromDirectory(soundDirectory);
        soundFile->setVisible(false);
        soundFile->addMouseListener(this, true);
    }
    if (vstFile != nullptr)
    {
        loadVSTFilesFromDirectory(vstDirectory);
        vstFile->setVisible(false);
        vstFile->addMouseListener(this, true);
    }
#pragma endregion
#pragma region Lambda
    soundFileButton.onClick = [this]()
        {currentMode = PanelMode::SoundMode;
    PanelSetting(currentMode);
        };
    vstFileButton.onClick = [this]()
        {currentMode = PanelMode::VSTMode;
    PanelSetting(currentMode);
        };
#pragma endregion
}

SoundSourcePanel::~SoundSourcePanel()
{
}

void SoundSourcePanel::paint (juce::Graphics& g)
{
  
}

void SoundSourcePanel::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..
    soundFileButton.setBounds(0, 0, 150, 50);
    vstFileButton.setBounds(150, 0, 150, 50);
    soundFile->setBounds(0,50,300,getHeight());
    vstFile->setBounds(0, 50, 300, getHeight());
}

void SoundSourcePanel::loadSoundFilesFromDirectory(const juce::File& directory)
{
    soundFile->soundPanel->items.clear();
    juce::DirectoryIterator iter(directory, false, "*", juce::File::TypesOfFileToFind::findFiles);
    while (iter.next())
    {
        juce::File file = iter.getFile();
        juce::String name = file.getFileName();
        if (name.endsWithIgnoreCase(".wav") || name.endsWithIgnoreCase(".mp3"))
        {
            DBG("File add: " + name);
            soundFile->addItem(file);
            DBG("model item:" << soundFile->soundPanel->items.size());
        }
    }
}

void SoundSourcePanel::loadVSTFilesFromDirectory(const juce::File& directory)
{
    vstFile->vstPanel->items.clear();
    auto flags = juce::File::findFilesAndDirectories | juce::File::ignoreHiddenFiles;
    //findFilesAndDirectories: file or directory transport
    //ignoreHiddenFiles: hide file and directory drop
    juce::DirectoryIterator iter(directory, true, "*.vst3", flags); //true is recursion
    while (iter.next())
    {
        juce::File file = iter.getFile();
        juce::String name = file.getFileName();
        if (file.hasFileExtension(".vst3"))
        {
            DBG("File add: " + name);
            vstFile->addItem(file);
            DBG("model item:" << vstFile->vstPanel->items.size());
        }
    }
}

void SoundSourcePanel::PanelSetting(PanelMode currentMode)
{
    if (currentMode == PanelMode::SoundMode)
    {
        addAndMakeVisible(*soundFile);
        vstFile->setVisible(false);
        soundFile->setVisible(true);
    }
    else if (currentMode == PanelMode::VSTMode)
    {
        addAndMakeVisible(*vstFile);
        soundFile->setVisible(false);
        vstFile->setVisible(true);
    }
}
