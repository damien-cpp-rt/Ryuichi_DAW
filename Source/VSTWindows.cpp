/*
  ==============================================================================

    VSTWindows.cpp
    Created: 23 Sep 2025 1:44:42pm
    Author:  KGA

  ==============================================================================
*/

#include "VSTWindows.h"
void PluginWindow::closeButtonPressed() 
{
    setVisible(false);
    if (onClose) juce::MessageManager::callAsync([cb = onClose] { cb(); });  
    //is MainComponent closeing error 
    //onClose override is close MainComponent close and plugin close is error
    //we want plugin callback fuction juce::MessageManager::callAsynce 
    // first here function defer
    // and second callback Lambda function starting
}