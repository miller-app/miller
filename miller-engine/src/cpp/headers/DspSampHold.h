/*
 *  Copyright 2017 Jacob Stern
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

#ifndef _DSP_SAMP_HOLD_H_
#define _DSP_SAMP_HOLD_H_

#include "DspObject.h"

/** [samphold~] */
class DspSampHold : public DspObject {
  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    DspSampHold(PdMessage *initMessage, PdGraph *graph);
    ~DspSampHold();

    static const char *getObjectLabel();
    std::string toString();

  private:
    float lastControlVal;
    float sample;

    void processMessage(int inletIndex, PdMessage *message);
    void processDspWithIndex(int fromIndex, int toIndex);
};

inline const char *DspSampHold::getObjectLabel() { return "samphold~"; }

inline std::string DspSampHold::toString() {
    return DspSampHold::getObjectLabel();
}

#endif // _DSP_SAMP_HOLD_H_
