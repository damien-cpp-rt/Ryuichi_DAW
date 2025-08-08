/*
  ==============================================================================

    VSTFileUI.h
    Created: 6 Aug 2025 4:52:28pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "VSTFilePanel.h"

//==============================================================================
/*
*/
class VSTFileUI  : public juce::Component, public juce::DragAndDropContainer
{
public:
    VSTFileUI();
    ~VSTFileUI() override;

    void paint (juce::Graphics&) override;
    void resized() override;
    void addItem(const juce::File& file);
    void mouseWheelMove(const juce::MouseEvent& event, const juce::MouseWheelDetails& wheel) override;
    juce::ListBox vstListBox;
    std::unique_ptr<VSTFilePanel> vstPanel = std::make_unique<VSTFilePanel>();
private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(VSTFileUI)
};
