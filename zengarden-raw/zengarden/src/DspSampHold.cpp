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

#include "DspSampHold.h"

/*
 * TODO: Implement discrete input for control signal
 */

MessageObject *DspSampHold::newObject(PdMessage *initMessage, PdGraph *graph) {
    return new DspSampHold(initMessage, graph);
}

DspSampHold::DspSampHold(PdMessage *initMessage, PdGraph *graph)
    : DspObject(2, 2, 0, 1, graph), lastControlVal(0), sample(0) {}

DspSampHold::~DspSampHold() {}

void DspSampHold::processMessage(int inletIndex, PdMessage *message) {
    // TODO
}

void DspSampHold::processDspWithIndex(int fromIndex, int toIndex) {
    for (int i = fromIndex; i < toIndex; i++) {
        float compare = dspBufferAtInlet[1][i];
        if (lastControlVal > compare) {
            sample = dspBufferAtInlet[0][i];
        }
        lastControlVal = compare;
        dspBufferAtOutlet[0][i] = sample;
    }
}
