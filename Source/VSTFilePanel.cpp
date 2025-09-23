/*
  ==============================================================================

    VSTFilePanel.cpp
    Created: 5 Aug 2025 10:45:22am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "VSTFilePanel.h"

//==============================================================================
VSTFilePanel::VSTFilePanel()
{
}

VSTFilePanel::~VSTFilePanel()
{
}

juce::var VSTFilePanel::getDragSourceDescription(const juce::SparseSet<int>& selectedRows)
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
void VSTFilePanel::paintListBoxItem(int rowNumber, juce::Graphics& g, int width, int height, bool rowIsSelected)
{
    g.fillAll(rowIsSelected ? juce::Colours::purple : juce::Colours::grey);
    g.setColour(juce::Colours::white);
    g.setFont(20.0f);
    g.drawText(items[rowNumber].getFileName(), 5, 0, width - 10, height, juce::Justification::centredLeft, true);
}
void VSTFilePanel::listBoxItemDoubleClicked(int row, const juce::MouseEvent&)
{
    if (row >= 0 && row < items.size())
        if (onDoubleClick) onDoubleClick(items[(int)row]);
}