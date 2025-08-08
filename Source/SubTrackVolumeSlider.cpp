/*
  ==============================================================================

    SubTrackVolumeSlider.cpp
    Created: 6 Aug 2025 3:37:55pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SubTrackVolumeSlider.h"

//==============================================================================
SubTrackVolumeSlider::SubTrackVolumeSlider()
{
    setSliderStyle(juce::Slider::LinearHorizontal); // ?? 수평 슬라이더
    setRange(0.0, 1.0, 0.01); // 볼륨 범위
    setValue(0.5); // 초기값
    setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0); // 텍스트 박스 숨김

    onValueChange = [this]()
        {
            DBG("볼륨 변경됨: " << getValue());
            // 여기서 Rust 쪽으로 값 전달하는 로직 넣으면 됨
        };
}

SubTrackVolumeSlider::~SubTrackVolumeSlider()
{
}

void SubTrackVolumeSlider::paint (juce::Graphics& g)
{
  
}

void SubTrackVolumeSlider::resized()
{
   
}
