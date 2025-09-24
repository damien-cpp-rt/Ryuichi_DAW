/*
  ==============================================================================

    AudioHostController.h
    Created: 23 Sep 2025 5:46:12pm
    Author:  KGA

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

enum class Backend : int {Cpal = 0, Juce = 1};
struct IAudioBackend {
    virtual ~IAudioBackend() = default;
    virtual bool start() = 0;
    virtual void stop() = 0;
};
std::unique_ptr<IAudioBackend> makeCpalBackend(/*엔진 핸들 등*/);
std::unique_ptr<IAudioBackend> makeJuceBackend(/*그래프/엔진 핸들 등*/);
//==============================================================================
/*
*/
class AudioHostController 
{
public:
    AudioHostController();
    ~AudioHostController();

    bool start(Backend preferred);
    void stop();

    /*Backend active() const { return activeBackend; }*/
private:

};
