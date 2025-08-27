#pragma once

#include <JuceHeader.h>
#include "SoundSourceFiles.h"
#include "MainTrack.h"
#include "Mixers.h"
#include "PlayBar.h"
#include "soundVectorData.h"
#include "AudioEngine.h"

#define FILEDRAG_DIR_PATH "C:/Ryuichi/UI_Image/FileDrag.png"
//==============================================================================
/*
    This component lives inside our window, and this is where you should put all
    your controls and content.
*/
class MainComponent  : public juce::AnimatedAppComponent, public juce::DragAndDropContainer
{
public:
    //==============================================================================
    MainComponent();
    ~MainComponent() override;

    //==============================================================================
    void paint (juce::Graphics&) override;
    void resized() override;
    void update() override;
    void mouseDrag(const juce::MouseEvent& e) override;
    void mouseDown(const juce::MouseEvent& e) override;
    void sourceMaxError();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_0 = std::make_shared<SoundCore::soundVecterData>();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_1 = std::make_shared<SoundCore::soundVecterData>();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_2 = std::make_shared<SoundCore::soundVecterData>();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_3 = std::make_shared<SoundCore::soundVecterData>();
private:
    //==============================================================================
    // Your private member variables go here...
    bool keyPressed(const juce::KeyPress& key) override;

    juce::String backGroundName = "Ryuichi";
    SoundSourceFiles soundBrowser;
    MainTrack mainTrack;
    Mixers mixers;
    PlayBar playBar;
    juce::Image fileDragIcon;
    std::unique_ptr<AudioEngine> audioEngine = std::make_unique<AudioEngine>();
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (MainComponent)
};
