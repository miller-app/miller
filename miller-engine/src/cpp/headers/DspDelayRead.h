/*
 *  Copyright 2009,2010 Reality Jockey, Ltd.
 *                 info@rjdj.me
 *                 http://rjdj.me/
 *
 *  This file is part of ZenGarden.
 *
 *  ZenGarden is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  ZenGarden is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with ZenGarden.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

#ifndef _DSP_DELAY_READ_H_
#define _DSP_DELAY_READ_H_

#include "DelayReceiver.h"

class DspDelayWrite;

/**
 * [delread~ symbol float]
 * This object also implements the <code>DelayReceiver</code> interface.
 */
class DspDelayRead : public DelayReceiver {

  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    DspDelayRead(PdMessage *initString, PdGraph *graph);
    ~DspDelayRead();

    static const char *getObjectLabel();
    std::string toString();
    ObjectType getObjectType();

    void onInletConnectionUpdate(unsigned int inletIndex);

  private:
    void processMessage(int inletIndex, PdMessage *message);
    static void processSignal(DspObject *dspObject, int fromIndex, int toIndex);

    float delayInSamples;
};

inline std::string DspDelayRead::toString() {
    return DspDelayRead::getObjectLabel();
}

inline const char *DspDelayRead::getObjectLabel() { return "delread~"; }

inline ObjectType DspDelayRead::getObjectType() { return DSP_DELAY_READ; }

#endif // _DSP_DELAY_READ_H_
