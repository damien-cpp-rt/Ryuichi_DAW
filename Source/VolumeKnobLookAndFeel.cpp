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

    // ��� ��
    g.setColour(juce::Colours::darkgrey);
    g.fillEllipse(bounds);

    // ���� ���� ��ĥ
    if (slider.getValue() != 0.0)
    {
        juce::Path valueArc;
        if (slider.getValue() > 0.0)
        {
            valueArc.addPieSegment(bounds, zeroAngle, angle, 0.8f);
            g.setColour(juce::Colours::purple); // ������ �� ��Ȳ��
        }
        else
        {
            valueArc.addPieSegment(bounds, angle, zeroAngle, 0.8f);
            g.setColour(juce::Colours::deepskyblue); // ���� �� �Ķ���
        }

        g.fillPath(valueArc);
    }
    // �׵θ�
    g.setColour(juce::Colours::black);
    g.drawEllipse(bounds, 1.0f);
}