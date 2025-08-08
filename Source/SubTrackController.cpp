/*
  ==============================================================================

    SubTrackController.cpp
    Created: 6 Aug 2025 1:12:09pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SubTrackController.h"

//==============================================================================
SubTrackController::SubTrackController()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
#pragma region BackGroundImg
    juce::File backGroundImgFile("C:/Ryuichi/UI_Image/SubTrackContorller.png");
    if (backGroundImgFile.existsAsFile())
    {
        subTrackContorllerBackGround = juce::ImageFileFormat::loadFrom(backGroundImgFile);
    }
#pragma endregion
#pragma region MuteButton
    juce::File onButtonFile("C:/Ryuichi/UI_Image/M_O.png");
    juce::File offButtonFile("C:/Ryuichi/UI_Image/M_C.png");
    if (onButtonFile.existsAsFile() && offButtonFile.existsAsFile())
    {
        juce::Image muteOnImg = juce::ImageFileFormat::loadFrom(onButtonFile);
        juce::Image muteOffImg = juce::ImageFileFormat::loadFrom(offButtonFile);

        muteToggleButton.setImages(muteOnImg, muteOffImg);
        addAndMakeVisible(muteToggleButton);
        muteToggleButton.setBounds(45,45, 20, 20);
    }
#pragma endregion
#pragma region Slider
    slider.setSliderStyle(juce::Slider::LinearHorizontal);
    slider.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
    slider.setRange(0.0, 1.0, 0.01);
    slider.setValue(0.5);
    addAndMakeVisible(slider);
#pragma endregion
}

SubTrackController::~SubTrackController()
{
}

void SubTrackController::paint (juce::Graphics& g)
{
    g.drawImage(subTrackContorllerBackGround, getLocalBounds().toFloat());
    g.setColour(juce::Colours::darkgrey);
    g.drawLine(5, 80, 105, 80, 5);
}

void SubTrackController::resized()
{
    slider.setBounds(10, 70, 90, 20);
}
