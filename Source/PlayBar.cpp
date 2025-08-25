/*
  ==============================================================================

    PlayBar.cpp
    Created: 8 Aug 2025 10:23:44am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "PlayBar.h"

//==============================================================================
PlayBar::PlayBar()
{
#pragma region TitileImag ,BPMImag
    juce::File titleFile(TITLE_DIR_PATH);
    if (titleFile.existsAsFile())
    {
        juce::Image titleImg = juce::ImageFileFormat::loadFrom(titleFile);
        titleImage.setImage(titleImg);
        addAndMakeVisible(titleImage);
    }
    juce::File bpmFile(BPMTEXT_DIR_PATH);
    if (bpmFile.existsAsFile())
    {
        juce::Image bpmTextImg = juce::ImageFileFormat::loadFrom(bpmFile);
        bpmTextImage.setImage(bpmTextImg);
       
    }
#pragma endregion
#pragma region PlayButton
    juce::File playOnFile(PLAY_ON_DIR_PATH);
    juce::File playOffFile(PLAY_OFF_DIR_PATH);
    if (playOnFile.existsAsFile() && playOffFile.existsAsFile())
    {
        juce::Image playOnImg = juce::ImageFileFormat::loadFrom(playOnFile);
        juce::Image playOffImg = juce::ImageFileFormat::loadFrom(playOffFile);

        playToggleButton.setImages(playOnImg, playOffImg);
        addAndMakeVisible(playToggleButton);
        playToggleButton.setBounds(0, 40, 50, 40);
    }
#pragma endregion
#pragma region StopButton
    juce::File stopOnFile(STOP_ON_DIR_PATH);
    juce::File stopOffFile(STOP_OFF_DIR_PATH);
    if (stopOnFile.existsAsFile() && stopOffFile.existsAsFile())
    {
        juce::Image stopOnImg = juce::ImageFileFormat::loadFrom(stopOnFile);
        juce::Image stopOffImg = juce::ImageFileFormat::loadFrom(stopOffFile);

        stopToggleButton.setImages(stopOnImg, stopOffImg);
        addAndMakeVisible(stopToggleButton);
        stopToggleButton.setBounds(50, 40, 50, 40);
    }
#pragma endregion
    addAndMakeVisible(bpm);
    addAndMakeVisible(bpmTextImage);
    bpmTextImage.toFront(true);
}

PlayBar::~PlayBar()
{
}

void PlayBar::paint (juce::Graphics& g)
{

}

void PlayBar::resized()
{
    titleImage.setBounds(0, 0, 300, 40);
    bpm.setBounds(100, 40, 200, 40);
    bpmTextImage.setBounds(100, 40, 40, 40);
}

