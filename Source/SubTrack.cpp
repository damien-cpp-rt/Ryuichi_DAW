/*
  ==============================================================================

    SubTrack.cpp
    Created: 6 Aug 2025 4:30:42pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SubTrack.h"

//==============================================================================
SubTrack::SubTrack()
{
#pragma region Img
    juce::File subTrackFile("C:/Ryuichi/UI_Image/Track_Note.png");
    if (subTrackFile.existsAsFile())
    {
        subTrackBackGround = juce::ImageFileFormat::loadFrom(subTrackFile);
    }
#pragma endregion
}

SubTrack::~SubTrack()
{
}

void SubTrack::paint (juce::Graphics& g)
{
    g.drawImage(subTrackBackGround, getLocalBounds().toFloat());
    
    if (soundTrackImg != nullptr)
    {
        int NextImage = 0;
        for (auto& img : *soundTrackImg)
        {
            g.drawImage(img,
                NextImage, 0,         // destination x, y
                100, 110,             // destination width, height
                0, 0,                 // source x, y
                img.getWidth(), img.getHeight()); // source width, height
            NextImage += 100;
        }
    }
}

void SubTrack::resized()
{
}

void SubTrack::mainTrackFileTransmission(const juce::String filePath)
{
    DBG("mainTrackFileTrans");
    juce::File droppedFile(filePath);
    if (droppedFile.existsAsFile())
    {
        juce::String fileName = droppedFile.getFileName();
        if (onFileDrepped)
        {
            onFileDrepped(droppedFile, fileName);
        }
    }
    else
    {
        DBG("File Find ERROR");
    }
}
