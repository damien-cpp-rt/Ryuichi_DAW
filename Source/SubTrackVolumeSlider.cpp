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
    setSliderStyle(juce::Slider::LinearHorizontal); // ?? ���� �����̴�
    setRange(0.0, 1.0, 0.01); // ���� ����
    setValue(0.5); // �ʱⰪ
    setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0); // �ؽ�Ʈ �ڽ� ����

    onValueChange = [this]()
        {
            DBG("���� �����: " << getValue());
            // ���⼭ Rust ������ �� �����ϴ� ���� ������ ��
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
