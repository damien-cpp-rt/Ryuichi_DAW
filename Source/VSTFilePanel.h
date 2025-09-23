/*
  ==============================================================================

    VSTFilePanel.h
    Created: 5 Aug 2025 10:45:22am
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

//==============================================================================
/*
*/
class VSTFilePanel  : public juce::ListBoxModel
{
public:
    VSTFilePanel();
    ~VSTFilePanel() override;
    std::function<void(const juce::File&)> onDoubleClick;
    void listBoxItemDoubleClicked(int row, const juce::MouseEvent&) override;
    int getNumRows()override
    {
        return items.size();
    }
    juce::var getDragSourceDescription(const juce::SparseSet<int>& selectedRows) override;
    void paintListBoxItem(int rowNumber, juce::Graphics& g, int width, int height, bool rowIsSelected) override;
    juce::Array<juce::File> items;
private:
    int lastClickedRow = -1;
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (VSTFilePanel)
};
