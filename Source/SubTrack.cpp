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
}

void SubTrack::resized()
{

}
void SubTrack::itemDropped(const juce::DragAndDropTarget::SourceDetails& dragSourceDetails)
{
    juce::String filePath = dragSourceDetails.description.toString();
    juce::File droppedFile(filePath);
    if (droppedFile.existsAsFile())
    {
        juce::String fileName = droppedFile.getFileName();
        soundBlockData makeBlockStruct{ droppedFile,fileName };
        soundFileArray.add(makeBlockStruct);
        //Rust Sound File Data Processing
    }
    else
    {
        DBG("File Find ERROR");
    }
}

bool SubTrack::isInterestedInDragSource(const SourceDetails& dragSourceDetails)
{
    return true;
}