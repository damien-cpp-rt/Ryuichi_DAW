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
#pragma endregion
#pragma region FileDrepped callBack
    
    if (mainTrack_0 != nullptr)
    {
        mainTrack.subTrack_0->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                DBG("onFileDrepped Lambda 0");
                if (mainTrack_0->fileNames.size() >= 10 || mainTrack_0->filePaths.size() >= 10 || mainTrack_0->soundWaveForm.size() >= 10)
                {
                    sourceMaxError();
                    return;
                }
                mainTrack_0->trackNumber = 0;
                mainTrack_0->filePaths.add(path);
                mainTrack_0->fileNames.add(name);
                const char* cPath = path.getFullPathName().toRawUTF8();
                const char* cName = name.toRawUTF8();
                const char* imgFile = audioEngine->rust_waveform_create(cPath, cName);
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
                DBG("onFileDrepped Lambda 1");
                if (mainTrack_1->fileNames.size() >= 10 || mainTrack_1->filePaths.size() >= 10 || mainTrack_1->soundWaveForm.size() >= 10)
                {
                    sourceMaxError();
                    return;
                }
                mainTrack_1->trackNumber = 1;
                mainTrack_1->filePaths.add(path);
                mainTrack_1->fileNames.add(name);
                const char* cPath = path.getFullPathName().toRawUTF8();
                const char* cName = name.toRawUTF8();
                const char* imgFile = audioEngine->rust_waveform_create(cPath, cName);
                juce::File waveFormFile(imgFile);
                if (waveFormFile.exists())
                {
                    juce::Image waveImg = juce::ImageFileFormat::loadFrom(waveFormFile);
                    mainTrack_1->soundWaveForm.add(waveImg);
                }
                audioEngine->rust_string_delete(const_cast<char*>(imgFile));
            };
    }
    if (mainTrack_2 != nullptr)
    {
        mainTrack.subTrack_2->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                DBG("onFileDrepped Lambda 2");
                if (mainTrack_2->fileNames.size() >= 10 || mainTrack_2->filePaths.size() >= 10 || mainTrack_2->soundWaveForm.size() >= 10)
                {
                    sourceMaxError();
                    return;
                }
                mainTrack_2->trackNumber = 2;
                mainTrack_2->filePaths.add(path);
                mainTrack_2->fileNames.add(name);
                const char* cPath = path.getFullPathName().toRawUTF8();
                const char* cName = name.toRawUTF8();
                const char* imgFile = audioEngine->rust_waveform_create(cPath, cName);
                juce::File waveFormFile(imgFile);
                if (waveFormFile.exists())
                {
                    juce::Image waveImg = juce::ImageFileFormat::loadFrom(waveFormFile);
                    mainTrack_2->soundWaveForm.add(waveImg);
                }
                audioEngine->rust_string_delete(const_cast<char*>(imgFile));
            };
    }
    if (mainTrack_3 != nullptr)
    {
        mainTrack.subTrack_3->onFileDrepped = [this](const juce::File& path, const juce::String& name)
            {
                DBG("onFileDrepped Lambda 3");
                if (mainTrack_3->fileNames.size() >= 10 || mainTrack_3->filePaths.size() >= 10 || mainTrack_3->soundWaveForm.size() >= 10)
                {
                    sourceMaxError();
                    return;
                }
                mainTrack_3->trackNumber = 3;
                mainTrack_3->filePaths.add(path);
                mainTrack_3->fileNames.add(name);
                const char* cPath = path.getFullPathName().toRawUTF8();
                const char* cName = name.toRawUTF8();
                const char* imgFile = audioEngine->rust_waveform_create(cPath, cName);
                juce::File waveFormFile(imgFile);
                if (waveFormFile.exists())
                {
                    juce::Image waveImg = juce::ImageFileFormat::loadFrom(waveFormFile);
                    mainTrack_3->soundWaveForm.add(waveImg);
                }
                audioEngine->rust_string_delete(const_cast<char*>(imgFile));
            };
    }
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
                break;
            case 2:
                DBG("1 Delete");
                mainTrack_1->fileNames.clear();
                mainTrack_1->filePaths.clear();
                mainTrack_1->soundWaveForm.clear();
                repaint();
                break;
            case 3:
                DBG("2 Delete");
                mainTrack_2->fileNames.clear();
                mainTrack_2->filePaths.clear();
                mainTrack_2->soundWaveForm.clear();
                repaint();
                break;
            case 4:
                DBG("3 Delete");
                mainTrack_3->fileNames.clear();
                mainTrack_3->filePaths.clear();
                mainTrack_3->soundWaveForm.clear();
                repaint();
                break;
            default:
                break;
            }
        };
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
    // This is called when the MainComponent is resized.
    // If you add any child components, this is where you should
    // update their positions.
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
}
void MainComponent::mouseDown(const juce::MouseEvent& e)
{
}

void MainComponent::sourceMaxError()
{
    juce::AlertWindow::showMessageBoxAsync(
        juce::AlertWindow::InfoIcon,
        "source excess!",
        "Source data can be stored in 10 locations");
}

