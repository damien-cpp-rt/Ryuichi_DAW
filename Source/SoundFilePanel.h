/*
  ==============================================================================

    SoundFilePanel.h
    Created: 5 Aug 2025 10:45:07am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class SoundFilePanel  : public juce::ListBoxModel
{
public:
    SoundFilePanel();
    ~SoundFilePanel() override;
    int getNumRows()override
    {
        DBG("getNumRows: " << items.size());
        return items.size();
    }
    juce::var getDragSourceDescription(const juce::SparseSet<int>& selectedRows) override;
    void paintListBoxItem(int rowNumber, juce::Graphics& g, int width, int height, bool rowIsSelected) override;
    juce::Array<juce::File> items;
private:
    int lastClickedRow = -1;
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SoundFilePanel)
};
