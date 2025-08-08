/*
  ==============================================================================

    SoundFilePanel.cpp
    Created: 5 Aug 2025 10:45:07am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SoundFilePanel.h"

//==============================================================================
SoundFilePanel::SoundFilePanel()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.

}

SoundFilePanel::~SoundFilePanel()
{
}

juce::var SoundFilePanel::getDragSourceDescription(const juce::SparseSet<int>& selectedRows)
{
    if (selectedRows.size() > 0)
    {
        lastClickedRow = selectedRows[0];
        if (lastClickedRow >= 0 && lastClickedRow < items.size())
        {
            return items[lastClickedRow].getFullPathName();
        }
    }
    return {};
}
void SoundFilePanel::paintListBoxItem(int rowNumber, juce::Graphics& g, int width, int height, bool rowIsSelected)
{
    g.fillAll(rowIsSelected ? juce::Colours::purple : juce::Colours::grey);
    g.setColour(juce::Colours::white);
    juce::Font Georgia("Georgia", 20.0f, juce::Font::plain);
    g.setFont(Georgia);
    g.drawText(items[rowNumber].getFileName(), 5, 0, width - 10, height, juce::Justification::centredLeft, true);
}

