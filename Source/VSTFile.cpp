/*
  ==============================================================================

    VSTFile.cpp
    Created: 5 Aug 2025 9:54:36am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "VSTFile.h"

//==============================================================================
VSTFile::VSTFile()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
    auto NormalImage = juce::Drawable::createFromImageFile(juce::File("C:/Ryuichi/UI_Image/VST_Plugin_N.png"));
    auto OverImage = juce::Drawable::createFromImageFile(juce::File("C:/Ryuichi/UI_Image/VST_Plugin_O.png"));
    auto DownImage = juce::Drawable::createFromImageFile(juce::File("C:/Ryuichi/UI_Image/VST_Plugin_C.png"));

    if (NormalImage != nullptr && OverImage != nullptr && DownImage != nullptr)
    {
        setImages(NormalImage.get(), OverImage.get(), DownImage.get(), nullptr);
    }
}

VSTFile::~VSTFile()
{
}

void VSTFile::paint (juce::Graphics& g)
{
    /* This demo code just fills the component's background and
       draws some placeholder text to get you started.

       You should replace everything in this method with your own
       drawing code..
    */
    DrawableButton::paint(g);
}

void VSTFile::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..

}
