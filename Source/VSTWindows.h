/*
  ==============================================================================

    VSTWindows.h
    Created: 23 Sep 2025 1:44:42pm
    Author:  KGA

  ==============================================================================
*/

#pragma once
#include <JuceHeader.h>

class PluginWindow : public juce::DocumentWindow
{
public:
    explicit PluginWindow(const juce::String& title)
        : juce::DocumentWindow(title,
            juce::Colours::black,
            juce::DocumentWindow::closeButton)
    {
        setUsingNativeTitleBar(true);
        setResizable(true, false);
    }

    std::function<void()> onClose; // 외부에서 람다로 정리 연결
    void closeButtonPressed() override;
};