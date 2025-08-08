#include "MainComponent.h"

//==============================================================================
MainComponent::MainComponent()
{
    setSize (600, 400);
    setFramesPerSecond(60);
    addAndMakeVisible(soundBrowser);
    addAndMakeVisible(mainTrack);
    addAndMakeVisible(mixers);
    addAndMakeVisible(playBar);
}

MainComponent::~MainComponent()
{
}

//==============================================================================
void MainComponent::paint (juce::Graphics& g)
{
    // (Our component is opaque, so we must completely fill the background with a solid colour)
    g.fillAll (juce::Colour::fromString("#2B2B2B"));
    juce::Font John("Segoe UI", 35.0f, juce::Font::italic);
    g.setFont (John);
    g.setColour (juce::Colours::black);
    g.drawText (backGroundName,getLocalBounds(), juce::Justification::centred, true);

#pragma region Animated
    g.setColour(juce::Colours::grey);

    float radiusX = 150.0f;
    float radiusY = 100.0f;
    float t = (float)getFrameCounter() * 0.06f;

    float x = getWidth() / 2.0f + radiusX * std::sin(t);
    float y = getHeight() / 2.0f + radiusY * std::sin(2 * t);

    float tPrev = t - 0.06f;
    float prevX = getWidth() / 2.0f + radiusX * std::sin(tPrev);
    float prevY = getHeight() / 2.0f + radiusY * std::sin(2 * tPrev);

    g.drawLine(prevX, prevY, x, y, 5.0f);
#pragma endregion
}
void MainComponent::update()
{

}
void MainComponent::resized()
{
    // This is called when the MainComponent is resized.
    // If you add any child components, this is where you should
    // update their positions.
    soundBrowser.setBounds(10, 10, 300, 1100);
    mainTrack.setBounds(600, 200, 1200, 640);
    mixers.setBounds(600, 850, 1200, 240);
    playBar.setBounds(1000, 0, 300, 80);
}
