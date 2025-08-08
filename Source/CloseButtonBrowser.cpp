/*
  ==============================================================================

    CloseButtonBrowser.cpp
    Created: 4 Aug 2025 3:13:52pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "CloseButtonBrowser.h"

//==============================================================================
CloseButtonBrowser::CloseButtonBrowser(): juce::DrawableButton("CloseButton", juce::DrawableButton::ButtonStyle::ImageOnButtonBackground)
{
    // In your constructor, you should add any child components, and
    // initialise any special settings that your component needs.
    auto Imag = juce::Drawable::createFromImageFile(juce::File("C:/Ryuichi/UI_Image/CloseButton.png"));
    if (Imag != nullptr)
    {
        setImages(Imag.get(), Imag.get(), Imag.get(), nullptr);
    }
}

CloseButtonBrowser::~CloseButtonBrowser()
{
}

void CloseButtonBrowser::paint (juce::Graphics& g)
{
    /* This demo code just fills the component's background and
       draws some placeholder text to get you started.

       You should replace everything in this method with your own
       drawing code..
    */
    DrawableButton::paint(g);
}

void CloseButtonBrowser::resized()
{
    // This method is where you should set the bounds of any child
    // components that your component contains..

}
