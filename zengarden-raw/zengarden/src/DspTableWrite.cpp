/*
 *  Copyright 2017 Jacob Stern
 *      jacob.stern@outlook.com
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

#include "DspTableWrite.h"
#include "PdGraph.h"

MessageObject *DspTableWrite::newObject(PdMessage *initMessage,
                                        PdGraph *graph) {
    return new DspTableWrite(initMessage, graph);
}

DspTableWrite::DspTableWrite(PdMessage *initMessage, PdGraph *graph)
    : DspObject(1, 1, 0, 0, graph), index(0), stopped(true), table(NULL) {
    name = initMessage->isSymbol(0)
               ? StaticUtils::copyString(initMessage->getSymbol(0))
               : NULL;
}

DspTableWrite::~DspTableWrite() { free(name); }

void DspTableWrite::setTable(MessageTable *aTable) { table = aTable; }

void DspTableWrite::processMessage(int inletIndex, PdMessage *message) {
    if (message->isBang(0)) {
        index = 0;
        stopped = false;
    } else {
        // TODO
    }
}

void DspTableWrite::processDspWithIndex(int fromIndex, int toIndex) {
    if (table != NULL && !stopped) {
        int bufferLength = 0;
        float *buffer = table->getBuffer(&bufferLength);
        if (index < bufferLength) {
            for (int i = fromIndex; i < toIndex; i++) {
                if (index >= bufferLength) {
                    break;
                }
                buffer[index] = dspBufferAtInlet[0][i];
                index++;
            }
        }
    }
}
