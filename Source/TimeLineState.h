/*
  ==============================================================================

    TimeLineState.h
    Created: 3 Sep 2025 11:01:13am
    Author:  KGA

  ==============================================================================
*/

#pragma once
#include <JuceHeader.h>
namespace TimeLine
{
    struct timeLineState {
        double sr = 48000.0;
        double bpm = 60.0;
        int    num = 4, den = 4;
        double pxPerBeat = 80.0;
        double scrollSamples = 0.0;

        double samplesPerBeat()   const { return sr * 60.0 / bpm; }
        double samplesPerPixel()  const { return samplesPerBeat() / pxPerBeat; }
        double xToSamples(float x)const { return scrollSamples + x * samplesPerPixel(); }
        float  samplesToX(double s)const { return float((s - scrollSamples) / samplesPerPixel()); }
        double snapSamples(double s, int gridDiv = 4) const {
            const double g = samplesPerBeat() / double(gridDiv);
            return std::round(s / g) * g;
        }
    };
}