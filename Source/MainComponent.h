#pragma once

#include <JuceHeader.h>
#include "SoundSourceFiles.h"
#include "MainTrack.h"
#include "Mixers.h"
#include "PlayBar.h"
#include "soundData.h"
#include "AudioEngine.h"
#include "ClipData.h"
#include "TimeLineState.h"

#define FILEDRAG_DIR_PATH "C:/Ryuichi/UI_Image/FileDrag.png"
struct AudioShared
{
    juce::AudioFormatManager   fm;
    juce::AudioThumbnailCache  cache{ 4096 };
    AudioShared() { fm.registerBasicFormats(); }
};
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
    void mouseUp(const juce::MouseEvent&) override;
    void mouseDown(const juce::MouseEvent& e) override;
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_0 = std::make_shared<SoundCore::soundVecterData>();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_1 = std::make_shared<SoundCore::soundVecterData>();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_2 = std::make_shared<SoundCore::soundVecterData>();
    std::shared_ptr<SoundCore::soundVecterData> mainTrack_3 = std::make_shared<SoundCore::soundVecterData>();
private:
    //==============================================================================
    // Your private member variables go here...
    juce::String backGroundName = "Ryuichi";
    SoundSourceFiles soundBrowser;
    MainTrack mainTrack;
    Mixers mixers;
    PlayBar playBar;
    juce::Image fileDragIcon;
    std::unique_ptr<AudioEngine> audioEngine = std::make_unique<AudioEngine>();

    float insertionX = 0.0f;
    AudioShared audioShared;
    TimeLine::timeLineState timeline;
    juce::OwnedArray<ClipData> clips[4];
    ClipData* selectedClip = nullptr; 
    int       selectedTrack = -1;      
    bool      isDraggingClip = false; 
    double    dragGrabOffsetS = 0.0;

    int       dragOrigTrack = -1;       // 드래그 시작 시 원본 위치
    uint64_t  dragOrigStart = 0;
    int       dragNewTrack = -1;       // 드래그 중 미리보기 최신 위치
    uint64_t  dragNewStart = 0;

    void mouseWheelMove(const juce::MouseEvent& e, const juce::MouseWheelDetails& w) override;
    bool keyPressed(const juce::KeyPress& key) override;
    void addClipToTrack(int track, const juce::File& file, uint64_t startSamples);
    void repaintTrack(int track);

    bool hitWhichTrackAndLocalX(const juce::MouseEvent& e, int& outTrack, float& outLocalX);
    int findClipIndexAtSample(int track, uint64_t s) const;
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (MainComponent)
};
