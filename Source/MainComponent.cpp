#include "MainComponent.h"
#include "AudioEngine.h"

//==============================================================================
MainComponent::MainComponent()
{
#pragma region Setting
    setSize (1200, 600);
    setFramesPerSecond(60);
    addAndMakeVisible(soundBrowser);
    addAndMakeVisible(mainTrack);
    mainTrack.setInterceptsMouseClicks(true, true);
    addAndMakeVisible(mixers);
    addAndMakeVisible(playBar);
    soundBrowser.addMouseListener(this, true);
    audioEngine->audioTrack_0 = mainTrack_0;
    audioEngine->audioTrack_1 = mainTrack_1;
    audioEngine->audioTrack_2 = mainTrack_2;
    audioEngine->audioTrack_3 = mainTrack_3;
    juce::File fileDragFile(FILEDRAG_DIR_PATH);
    if (fileDragFile.existsAsFile())
    {
        fileDragIcon = juce::ImageFileFormat::loadFrom(fileDragFile);
    }
    addMouseListener(this, true);
#pragma endregion
#pragma region FileDrepped callBack
    mainTrack.onDropIntoSubTrack = [this](int track, const juce::File& file, float laneX)
        {
            const double s = timeline.xToSamples(laneX);
            const uint64_t startS = (uint64_t)timeline.snapSamples(s, 4);
            addClipToTrack(track, file, startS);
        };
#pragma endregion
#pragma region SubTrackImg reference
    mainTrack.subTrack_0->soundTrackImg = &(mainTrack_0->soundWaveForm);
    mainTrack.subTrack_1->soundTrackImg = &(mainTrack_1->soundWaveForm);
    mainTrack.subTrack_2->soundTrackImg = &(mainTrack_2->soundWaveForm);
    mainTrack.subTrack_3->soundTrackImg = &(mainTrack_3->soundWaveForm);
#pragma endregion
#pragma region TrackClear
    mainTrack.handleMenuSelection = [this](int selectedId) 
        {
            switch (selectedId)
            {
            case 1:
                DBG("0 Delete");
                mainTrack_0->fileNames.clear();
                mainTrack_0->filePaths.clear();
                mainTrack_0->soundWaveForm.clear();
                repaint();
                if (audioEngine->rust_file_all_delete(0)) {
                    DBG("[Rust]-file_all_delete");
                }
                break;
            case 2:
                DBG("1 Delete");
                mainTrack_1->fileNames.clear();
                mainTrack_1->filePaths.clear();
                mainTrack_1->soundWaveForm.clear();
                repaint();
                if (audioEngine->rust_file_all_delete(1)) {
                    DBG("[Rust]-file_all_delete");
                }
                break;
            case 3:
                DBG("2 Delete");
                mainTrack_2->fileNames.clear();
                mainTrack_2->filePaths.clear();
                mainTrack_2->soundWaveForm.clear();
                repaint();
                if (audioEngine->rust_file_all_delete(2)) {
                    DBG("[Rust]-file_all_delete");
                }
                break;
            case 4:
                DBG("3 Delete");
                mainTrack_3->fileNames.clear();
                mainTrack_3->filePaths.clear();
                mainTrack_3->soundWaveForm.clear();
                repaint();
                if (audioEngine->rust_file_all_delete(3)) {
                    DBG("[Rust]-file_all_delete");
                }
                break;
            default:
                break;
            }
        };
#pragma endregion
#pragma region SubTrackController_Volume_and_Mut_and_BPM
    mainTrack.subTrackController_0->slider.onValueChange = [this]() {
        mainTrack_0->volume = mainTrack.subTrackController_0->slider.getValue();
        if (audioEngine->rust_volume_update(mainTrack_0->volume, 0)) {
            DBG("[volume_Update]-Ok");
        }
        else {
            DBG("[volume_Update]-Error");
        }
        };
    mainTrack.subTrackController_1->slider.onValueChange = [this]() {
        mainTrack_1->volume = mainTrack.subTrackController_1->slider.getValue();
        if(audioEngine->rust_volume_update(mainTrack_1->volume, 1)) {
            DBG("[volume_Update]-Ok");
        }
        else {
            DBG("[volume_Update]-Error");
        }
        };
    mainTrack.subTrackController_2->slider.onValueChange = [this]() {
        mainTrack_2->volume = mainTrack.subTrackController_2->slider.getValue();
        if (audioEngine->rust_volume_update(mainTrack_2->volume, 2)) {
            DBG("[volume_Update]-Ok");
        }
        else {
            DBG("[volume_Update]-Error");
        }
        };
    mainTrack.subTrackController_3->slider.onValueChange = [this]() {
        mainTrack_3->volume = mainTrack.subTrackController_3->slider.getValue();
        if(audioEngine->rust_volume_update(mainTrack_3->volume, 3)) {
            DBG("[volume_Update]-Ok");
        }
        else {
            DBG("[volume_Update]-Error");
        }
        };


    mainTrack.subTrackController_0->muteToggleButton.onClick = [this]() {
        mainTrack_0->isMuted = mainTrack.subTrackController_0->muteToggleButton.getToggleState();
        if (audioEngine->rust_mute_update(mainTrack_0->isMuted,0)) {
            DBG("[Mute_Update]-Ok");
        }
        else {
            DBG("[Mute_Update]-Error");
        }
        };
    mainTrack.subTrackController_1->muteToggleButton.onClick = [this]() {
        mainTrack_1->isMuted = mainTrack.subTrackController_1->muteToggleButton.getToggleState();
        if (audioEngine->rust_mute_update(mainTrack_1->isMuted, 1)) {
            DBG("[Mute_Update]-Ok");
        }
        else {
            DBG("[Mute_Update]-Error");
        }
        };
    mainTrack.subTrackController_2->muteToggleButton.onClick = [this]() {
        mainTrack_2->isMuted = mainTrack.subTrackController_2->muteToggleButton.getToggleState();
        if (audioEngine->rust_mute_update(mainTrack_2->isMuted, 2)) {
            DBG("[Mute_Update]-Ok");
        }
        else {
            DBG("[Mute_Update]-Error");
        }
        };
    mainTrack.subTrackController_3->muteToggleButton.onClick = [this]() {
        mainTrack_3->isMuted = mainTrack.subTrackController_3->muteToggleButton.getToggleState();
        if (audioEngine->rust_mute_update(mainTrack_3->isMuted, 3)) {
            DBG("[Mute_Update]-Ok");
        }
        else {
            DBG("[Mute_Update]-Error");
        }
        };
    playBar.bpm.bpmEditor.onTextChange = [this]() {

        const float newBpm = playBar.bpm.bpmEditor.getText().getFloatValue();
        if (newBpm > 0.0f && audioEngine->rust_bpm_update(newBpm)) {
            timeline.bpm = newBpm;
            if (mainTrack.subTrack_0) mainTrack.subTrack_0->repaint();
            if (mainTrack.subTrack_1) mainTrack.subTrack_1->repaint();
            if (mainTrack.subTrack_2) mainTrack.subTrack_2->repaint();
            if (mainTrack.subTrack_3) mainTrack.subTrack_3->repaint();
            playBar.repaint();
            DBG("[BPM_Update]-Ok");
        } else {
            DBG("[BPM_Update]-Error");
        }
        };
#pragma endregion
#pragma region MixerController_Pan_R_D
    mixers.trackMixer_0->volumeKnob.onValueChange = [this]() {
        mainTrack_0->pan = mixers.trackMixer_0->volumeKnob.getValue();
        if (audioEngine->rust_pan_update(mainTrack_0->pan, 0)) {
            DBG("[Pan_Update]-Ok");
        }
        else {
            DBG("[Pan_Update]-Error");
        }
        };
    mixers.trackMixer_1->volumeKnob.onValueChange = [this]() {
        mainTrack_1->pan = mixers.trackMixer_1->volumeKnob.getValue();
        if (audioEngine->rust_pan_update(mainTrack_1->pan, 1)) {
            DBG("[Pan_Update]-Ok");
        }
        else {
            DBG("[Pan_Update]-Error");
        }
        };
    mixers.trackMixer_2->volumeKnob.onValueChange = [this]() {
        mainTrack_2->pan = mixers.trackMixer_2->volumeKnob.getValue();
        if (audioEngine->rust_pan_update(mainTrack_2->pan, 2)) {
            DBG("[Pan_Update]-Ok");
        }
        else {
            DBG("[Pan_Update]-Error");
        }
        };
    mixers.trackMixer_3->volumeKnob.onValueChange = [this]() {
        mainTrack_3->pan = mixers.trackMixer_3->volumeKnob.getValue();
        if (audioEngine->rust_pan_update(mainTrack_3->pan, 3)) {
            DBG("[Pan_Update]-Ok");
        }
        else {
            DBG("[Pan_Update]-Error");
        }
        };
#pragma endregion
#pragma region bind_Data
    mainTrack.subTrack_0->bindTimeline(&timeline);
    mainTrack.subTrack_1->bindTimeline(&timeline);
    mainTrack.subTrack_2->bindTimeline(&timeline);
    mainTrack.subTrack_3->bindTimeline(&timeline);

    mainTrack.subTrack_0->bindClips(&clips[0]);
    mainTrack.subTrack_1->bindClips(&clips[1]);
    mainTrack.subTrack_2->bindClips(&clips[2]);
    mainTrack.subTrack_3->bindClips(&clips[3]);
#pragma endregion
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
    soundBrowser.setBounds(10, 10, 300, 1100);
    mainTrack.setBounds(600, 200, 1200, 640);
    mixers.setBounds(600, 850, 1200, 240);
    playBar.setBounds(1000, 0, 300, 80);
}

void MainComponent::mouseDrag(const juce::MouseEvent& e)
{
    auto* listBox = &soundBrowser.sourcePanel.soundFile->soundListBox;

    // 마우스 위치를 listBox 기준 좌표로 변환
    auto relativePos = e.getEventRelativeTo(listBox).position.toInt();

    // 마우스가 listBox 내부에 있을 때만 드래그 시작
    if (listBox->getLocalBounds().contains(relativePos))
    {

        auto dragDescription = soundBrowser.sourcePanel.soundFile->soundPanel->getDragSourceDescription(
            listBox->getSelectedRows());
        startDragging(dragDescription, listBox, fileDragIcon, true);
        
    }

    if (!isDraggingClip || !selectedClip) return;
    int hitTrack;
    float localX;
    if (!hitWhichTrackAndLocalX(e, hitTrack, localX)) return;

    const double sNow = timeline.xToSamples(localX);
    double newStart = sNow - dragGrabOffsetS;
    newStart = timeline.snapSamples(newStart, 4);
    newStart = std::max(0.0, newStart);
    if (hitTrack != selectedTrack)
    {
        const int oldTrack = selectedTrack;
        const int oldIdx = clips[selectedTrack].indexOf(selectedClip);
        if (oldIdx >= 0) {
            auto* p = clips[selectedTrack].removeAndReturn(oldIdx);
            clips[hitTrack].add(p);
            selectedTrack = hitTrack;
            repaintTrack(oldTrack);
        }
    }
    selectedClip->startS = (uint64_t)std::llround(newStart);
    repaintTrack(selectedTrack);
}
void MainComponent::mouseUp(const juce::MouseEvent&)
{
    isDraggingClip = false;
}
void MainComponent::mouseDown(const juce::MouseEvent& e)
{
    int hitTrack; float localX;
    if (!hitWhichTrackAndLocalX(e, hitTrack, localX)) return;

    const double s = timeline.xToSamples(localX);
    const uint64_t sClamped = (uint64_t)std::max(0.0, s);

    if (e.mods.isRightButtonDown() || e.mods.isPopupMenu()) //mouseRight Delete
    {
        const int idx = findClipIndexAtSample(hitTrack, sClamped);
        if (idx >= 0)
        {
            ClipData* victim = clips[hitTrack][idx];
            const uint64_t victimStart = victim->startS;

            if (audioEngine && audioEngine->rust_file_delet(hitTrack, victimStart)) {

                clips[hitTrack].remove(idx);              // OwnedArray is RAW delete
                if (selectedClip == victim) {             // Selecteding the delet
                    selectedClip = nullptr; selectedTrack = -1; isDraggingClip = false;
                }
                repaintTrack(hitTrack); //track 
            }
            else {
                DBG("rust_sound_delete_clip_by_start failed");
            }
            return; // 우클릭은 여기서 끝
        }

    }

    const int idx = findClipIndexAtSample(hitTrack, sClamped);
    if (idx < 0) {
        // 빈 공간 클릭: 선택 해제
        selectedClip = nullptr; selectedTrack = -1; isDraggingClip = false;
        return;
    }

    // 이 지점에 있는 클립을 집는다
    selectedClip = clips[hitTrack][idx];
    selectedTrack = hitTrack;
    isDraggingClip = true;

    // 클릭 지점이 클립 시작으로부터 얼마나 떨어져 있는지(샘플) 저장
    dragGrabOffsetS = (double)sClamped - (double)selectedClip->startS;

}

bool MainComponent::keyPressed(const juce::KeyPress& key)
{
    if (key.getTextCharacter() == '+') timeline.pxPerBeat = juce::jmin(800.0, timeline.pxPerBeat * 1.2);
    if (key.getTextCharacter() == '-') timeline.pxPerBeat = juce::jmax(10.0, timeline.pxPerBeat / 1.2);

    if (key.getKeyCode() == juce::KeyPress::spaceKey)
    {
        const bool newPlayState = !playBar.playToggleButton.getToggleState();
        playBar.playToggleButton.setToggleState(newPlayState, juce::sendNotification);

        const bool newStopState = !playBar.stopToggleButton.getToggleState();
        playBar.stopToggleButton.setToggleState(newStopState, juce::sendNotification);

        if (playBar.playToggleButton.getToggleState())
        {
            audioEngine->rust_start_sound(true);
            return true;
        }
        else
        {
            audioEngine->rust_start_sound(false);
            return false;
        }
    }

    mainTrack.subTrack_0->repaint();
    mainTrack.subTrack_1->repaint();
    mainTrack.subTrack_2->repaint();
    mainTrack.subTrack_3->repaint();
    return true;
}

double MainComponent::readSeconds(const juce::File& f)
{
    std::unique_ptr<juce::AudioFormatReader> r(audioShared.fm.createReaderFor(f));
    return r ? (double)r->lengthInSamples / r->sampleRate : 0.0;
}

void MainComponent::addClipToTrack(int track, const juce::File& file, uint64_t startSamples)
{
    if (track < 0 || track >= 4) return;
    if (!file.existsAsFile())     return;

    // 파일 길이 → 타임라인 SR 기준 길이(샘플)로 변환
    const double sec = readSeconds(file); // 디코딩 작업 초수를 수집
    const uint64_t lenS = (uint64_t)std::llround(sec * timeline.sr); //타임라인 기준으로 초당 샘플 몇개 필요한지 만들고 반올림

    // 클립 생성/보관 (AudioThumbnail 소유)
    auto* c = new ClipData(audioShared.fm, audioShared.cache, file, startSamples, lenS);
    clips[track].add(c);

    // (선택) 엔진에 등록
    if (audioEngine) {
        const char* path = file.getFullPathName().toRawUTF8();
        const uint32_t srcSR = 48000; // 파일 실제 SR을 알고 있으면 그 값으로
        audioEngine->rust_file_update(track, path,startSamples, lenS, srcSR);
    }

    repaintTrack(track);
}

void MainComponent::repaintTrack(int track)
{
    if (track == 0 && mainTrack.subTrack_0) mainTrack.subTrack_0->repaint();
    else if (track == 1 && mainTrack.subTrack_1) mainTrack.subTrack_1->repaint();
    else if (track == 2 && mainTrack.subTrack_2) mainTrack.subTrack_2->repaint();
    else if (track == 3 && mainTrack.subTrack_3) mainTrack.subTrack_3->repaint();
}


void MainComponent::mouseWheelMove(const juce::MouseEvent& e, const juce::MouseWheelDetails& w)
{
    if (e.mods.isCtrlDown()) {
     
        timeline.pxPerBeat = juce::jlimit(10.0, 800.0, timeline.pxPerBeat * (1.0 + w.deltaY * 0.2));
    }
    mainTrack.subTrack_0->repaint(); mainTrack.subTrack_1->repaint();
    mainTrack.subTrack_2->repaint(); mainTrack.subTrack_3->repaint();
}
bool MainComponent::hitWhichTrackAndLocalX(const juce::MouseEvent& e, int& outTrack, float& outLocalX)
{
    auto p = e.getEventRelativeTo(&mainTrack).position.toInt();

    struct Lane { juce::Component* comp; int index; } lanes[] = {
       { mainTrack.subTrack_0.get(), 0 },
       { mainTrack.subTrack_1.get(), 1 },
       { mainTrack.subTrack_2.get(), 2 },
       { mainTrack.subTrack_3.get(), 3 },
    };

    for (auto& L : lanes) {
        if (!L.comp) continue;
        auto local = L.comp->getLocalPoint(&mainTrack, p);
        if (L.comp->getLocalBounds().contains(local)) {
            outTrack = L.index;
            outLocalX = (float)local.x;
            return true;
        }
    }
    return false;
}

int MainComponent::findClipIndexAtSample(int track, uint64_t s) const
{
    const auto& arr = clips[track];
    for (int i = 0; i < arr.size(); ++i) {
        if (auto* c = arr[i]) {
            if (s >= c->startS && s < c->startS + c->lenS)
            {
                return i;
            }
        }
    }
    return -1;
}
