#include "MainComponent.h"
#include "AudioEngine.h"

//==============================================================================
MainComponent::MainComponent()
{
    setSize (600, 400);
    setFramesPerSecond(60);
    addAndMakeVisible(soundBrowser);
    addAndMakeVisible(mainTrack);
    addAndMakeVisible(mixers);
    addAndMakeVisible(playBar);
    audioEngine->audioTrack_0 = mainTrack_0;
    audioEngine->audioTrack_1 = mainTrack_1;
    audioEngine->audioTrack_2 = mainTrack_2;
    audioEngine->audioTrack_3 = mainTrack_3;
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
void MainComponent::fileDropped()
{
    if (mainTrack_0 != nullptr)
    {
        mainTrack.subTrack_0->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                mainTrack_0->trackNumber = 0;
                mainTrack_0->filePaths.add(path);
                mainTrack_0->fileNames.add(name);
                const char* cPath = path.getFullPathName().toRawUTF8();
                const char* cName = name.toRawUTF8();
                const char* imgFile=audioEngine->rust_waveform_create(cPath, cName);
                juce::File waveFormFile(imgFile);
                if (waveFormFile.exists())
                {
                    juce::Image waveImg = juce::ImageFileFormat::loadFrom(waveFormFile);
                    mainTrack_0->soundWaveForm.add(waveImg);
                }
                audioEngine->rust_string_delete(const_cast<char*>(imgFile));
            };
    }
    if (mainTrack_1 != nullptr)
    {
        mainTrack.subTrack_1->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                mainTrack_1->trackNumber = 1;
                mainTrack_1->filePaths.add(path);
                mainTrack_1->fileNames.add(name);
            };
    }
    if (mainTrack_2 != nullptr)
    {
        mainTrack.subTrack_2->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                mainTrack_2->trackNumber = 2;
                mainTrack_2->filePaths.add(path);
                mainTrack_2->fileNames.add(name);
            };
    }
    if (mainTrack_3 != nullptr)
    {
        mainTrack.subTrack_3->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                mainTrack_3->trackNumber = 3;
                mainTrack_3->filePaths.add(path);
                mainTrack_3->fileNames.add(name);
            };
    }
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
