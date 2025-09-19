/*
  ==============================================================================

    AssetsPath.h
    Created: 17 Sep 2025 2:53:46pm
    Author:  KGA

  ==============================================================================
*/

#pragma once
#include <JuceHeader.h>

namespace Path
{
    inline juce::File exeDir()
    {
        auto exe = juce::File::getSpecialLocation(juce::File::currentExecutableFile);
        return exe.getParentDirectory();
    }

    inline juce::File assetsDir()
    {
        return exeDir().getChildFile("Assets");
    }
}
