/*
  ==============================================================================

    SubTrack.cpp
    Created: 6 Aug 2025 4:30:42pm
    Author:  KGA

  ==============================================================================
*/

#include <JuceHeader.h>
#include "SubTrack.h"

//==============================================================================
SubTrack::SubTrack()
{
#pragma region Img
    juce::File subTrackFile("C:/Ryuichi/UI_Image/Track_Note.png");
    if (subTrackFile.existsAsFile())
    {
        subTrackBackGround = juce::ImageFileFormat::loadFrom(subTrackFile);
    }
#pragma endregion
}

SubTrack::~SubTrack()
{
}

void SubTrack::paint (juce::Graphics& g)
{
    g.drawImage(subTrackBackGround, getLocalBounds().toFloat());

    if (timeline != nullptr)
        drawBeatGrid(g, getLocalBounds());
    if (clips && timeline) {
        for (auto* c : *clips) {
            const int x = (int)std::floor(timeline->samplesToX((double)c->startS));
            const int w = (int)std::ceil((double)c->lenS / timeline->samplesPerPixel());
            juce::Rectangle<int> r(x, 0, juce::jmax(8, w), getHeight());

            g.setColour(juce::Colours::dimgrey.darker(0.2f)); 
            g.fillRoundedRectangle(r.toFloat(), 4.0f);
            g.setColour(juce::Colours::white.withAlpha(0.35f)); 
            g.drawRoundedRectangle(r.toFloat(), 4.0f, 1.0f);

            if (c->thumb && c->thumb->getTotalLength() > 0.0) {
                g.setColour(juce::Colours::lime);
                auto drawArea = r.reduced(2);               // Rectangle<int>
                c->thumb->drawChannels(g, drawArea, 0.0, c->thumb->getTotalLength(), 1.0f);
            }
        }
    }
}

void SubTrack::resized()
{
}


void SubTrack::drawBeatGrid(juce::Graphics& g, juce::Rectangle<int> area)
{
    const double spb = timeline->samplesPerBeat();
    const double s0 = timeline->xToSamples((float)area.getX());
    const double s1 = timeline->xToSamples((float)area.getRight());
    const long long b0 = (long long)std::floor(s0 / spb) - 1;
    const long long b1 = (long long)std::ceil(s1 / spb) + 1;

    const double pxPerBar = timeline->pxPerBeat * timeline->num; // num=박자수(보통 4)
    const bool showLabels = pxPerBar > 40.0;
    const bool showSub = pxPerBar > 20.0;

    for (long long b = b0; b <= b1; ++b)
    {
        const bool isBar = (timeline->num > 0) ? ((b % timeline->num) == 0) : (b == 0);
        const float x = timeline->samplesToX((double)b * spb);

        if (isBar) {
            g.setColour(juce::Colours::white.withAlpha(0.20f));
            g.drawVerticalLine((int)std::round(x), area.getY(), area.getBottom());
            if (showLabels) {
                g.setColour(juce::Colours::white.withAlpha(0.9f));
                g.drawText("Bar " + juce::String((int)(b / timeline->num) + 1),
                    (int)x + 3, area.getY() + 2, 60, 16, juce::Justification::left, false);
            }
        }
        else {
            g.setColour(juce::Colours::white.withAlpha(0.10f));
            g.drawVerticalLine((int)std::round(x), area.getY(), area.getBottom());
        }

        if (showSub) {
            for (int k = 1; k < 4; ++k) {
                const double subs = (double)b * spb + k * (spb / 4.0);
                const float xs = timeline->samplesToX(subs);
                g.setColour(juce::Colours::white.withAlpha(0.06f));
                g.drawVerticalLine((int)std::round(xs), area.getY(), area.getBottom());
            }
        }
    }
}
