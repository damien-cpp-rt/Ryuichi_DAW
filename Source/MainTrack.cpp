/*
  ==============================================================================

    MainTrack.cpp
    Created: 5 Aug 2025 5:45:23pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "MainTrack.h"

//==============================================================================
MainTrack::MainTrack()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
#pragma region Imag
    juce::File windowImg("C:/Ryuichi/UI_Image/TrackBar.png"); //Àý´ë°æ·Î ºÒ·¯¿À±â
    if (windowImg.existsAsFile()) //Á¤»óÀÎÁö Á¶È¸
    {
        juce::Image img = juce::ImageFileFormat::loadFrom(windowImg); //image ÀúÀå
        WindowBarComponent.setImage(img);//ÀÌ¹ÌÁö º¯¼ö¿¡ ÀúÀå
        addAndMakeVisible(&WindowBarComponent); //ÀÌ¹ÌÁö ¶ì¿ì±â
    }
    juce::File mainTrackImg("C:/Ryuichi/UI_Image/TrackBackGround.png");
    if (mainTrackImg.existsAsFile())
    {
        juce::Image img = juce::ImageFileFormat::loadFrom(mainTrackImg);
        mainTrackBackGround.setImage(img);
        addAndMakeVisible(&mainTrackBackGround);
    }
#pragma endregion
#pragma region SubTrack
    addAndMakeVisible(subTrack_0.get());
    addAndMakeVisible(subTrack_1.get());
    addAndMakeVisible(subTrack_2.get());
    addAndMakeVisible(subTrack_3.get());
    addAndMakeVisible(subTrackController_0.get());
    addAndMakeVisible(subTrackController_1.get());
    addAndMakeVisible(subTrackController_2.get());
    addAndMakeVisible(subTrackController_3.get());
#pragma endregion
#pragma region CloseButton
    if (mainTrackCloseButton != nullptr)
    {
        addAndMakeVisible(mainTrackCloseButton.get());
        setVisible(true);
    mainTrackCloseButton->onClick = [this]()
        {
            DBG("MainTrackExit");
            setVisible(false);
        };
    }
#pragma endregion
}

MainTrack::~MainTrack()
{
}

void MainTrack::paint (juce::Graphics& g)
{
   
}

void MainTrack::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..
#pragma region Imag or CloseButton
    WindowBarComponent.setBounds(0, 0, 1200, 40);
    mainTrackCloseButton->setBounds(1160, 5, 30, 30);
    mainTrackBackGround.setBounds(0, 40, 1200, 600);
#pragma endregion
#pragma region SubTrackController
    subTrackController_0->setBounds(1, 105, 110, 110);
    subTrackController_1->setBounds(1, 220, 110, 110);
    subTrackController_2->setBounds(1, 335, 110, 110);
    subTrackController_3->setBounds(1, 450, 110, 110);
    subTrack_0->setBounds(109, 105, 1090, 110);
    subTrack_1->setBounds(109, 220, 1090, 110);
    subTrack_2->setBounds(109, 335, 1090, 110);
    subTrack_3->setBounds(109, 450, 1090, 110);
#pragma endregion
}

