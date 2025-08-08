/*
  ==============================================================================

    VolumeKnobLookAndFeel.cpp
    Created: 8 Aug 2025 2:08:14pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "VolumeKnobLookAndFeel.h"

//==============================================================================
VolumeKnobLookAndFeel::VolumeKnobLookAndFeel()
{

}

VolumeKnobLookAndFeel::~VolumeKnobLookAndFeel()
{
}
void VolumeKnobLookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
    float sliderPosProportional, float rotaryStartAngle,
    float rotaryEndAngle, juce::Slider& slider)
{
    auto bounds = juce::Rectangle<float>(x, y, width, height);
    auto radius = juce::jmin(bounds.getWidth(), bounds.getHeight()) / 2.0f;
    auto center = bounds.getCentre();

    float zeroPos = 0.5f; 
    float angle = rotaryStartAngle + sliderPosProportional * (rotaryEndAngle - rotaryStartAngle);
    float zeroAngle = rotaryStartAngle + zeroPos * (rotaryEndAngle - rotaryStartAngle);

    // 배경 원
    g.setColour(juce::Colours::darkgrey);
    g.fillEllipse(bounds);

    // 값에 따라 색칠
    if (slider.getValue() != 0.0)
    {
        juce::Path valueArc;
        if (slider.getValue() > 0.0)
        {
            valueArc.addPieSegment(bounds, zeroAngle, angle, 0.8f);
            g.setColour(juce::Colours::purple); // 오른쪽 → 주황색
        }
        else
        {
            valueArc.addPieSegment(bounds, angle, zeroAngle, 0.8f);
            g.setColour(juce::Colours::deepskyblue); // 왼쪽 → 파란색
        }

        g.fillPath(valueArc);
    }
    // 테두리
    g.setColour(juce::Colours::black);
    g.drawEllipse(bounds, 1.0f);
}