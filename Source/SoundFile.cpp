/*
  ==============================================================================

    SoundFile.cpp
    Created: 5 Aug 2025 9:53:56am
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SoundFile.h"

//==============================================================================
SoundFile::SoundFile()
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
    
        
        
    auto NormalImage = juce::Drawable::createFromImageFile(juce::File((Path::assetsDir().getChildFile("UI_Image").getChildFile("Sound_Files_N.png"))));
    auto OverImage = juce::Drawable::createFromImageFile(juce::File((Path::assetsDir().getChildFile("UI_Image").getChildFile("Sound_Files_O.png"))));
    auto DownImage = juce::Drawable::createFromImageFile(juce::File((Path::assetsDir().getChildFile("UI_Image").getChildFile("Sound_Files_C.png"))));

    if (NormalImage != nullptr && OverImage != nullptr && DownImage != nullptr)
    {
        setImages(NormalImage.get(), OverImage.get(), DownImage.get(), nullptr);
    }
}

SoundFile::~SoundFile()
{
}

void SoundFile::paint (juce::Graphics& g)
{
    /* This demo code just fills the component's background and
       draws some placeholder text to get you started.

       You should replace everything in this method with your own
       drawing code..
    */

    DrawableButton::paint(g);
}

void SoundFile::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..

}
