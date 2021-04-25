/*
 *  Copyright 2009,2010,2011,2012 Reality Jockey, Ltd.
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

#include "DspOsc.h"
#include "PdGraph.h"
#include <cmath>

#define COS_TABLE_SIZE 32768

// initialise the static class variables
float *DspOsc::cos_table = NULL;
int DspOsc::refCount = 0;

MessageObject *DspOsc::newObject(PdMessage *initMessage, PdGraph *graph) {
    return new DspOsc(initMessage, graph);
}

DspOsc::DspOsc(PdMessage *initMessage, PdGraph *graph)
    : DspObject(2, 2, 0, 1, graph) {
    frequency = initMessage->isFloat(0) ? initMessage->getFloat(0) : 440.0f;
    phase = 0.0f;
    refCount++;

    if (cos_table == NULL) {
        cos_table = ALLOC_ALIGNED_BUFFER(COS_TABLE_SIZE * sizeof(float));
        for (int i = 0; i < COS_TABLE_SIZE; i++) {
            cos_table[i] =
                cosf(2.0f * M_PI * ((float)i) / (COS_TABLE_SIZE - 1));
        }
    }

    processFunction = &processScalar;
}

DspOsc::~DspOsc() {
    if (--refCount == 0) {
        FREE_ALIGNED_BUFFER(cos_table);
        cos_table = NULL;
    }
}

void DspOsc::onInletConnectionUpdate(unsigned int inletIndex) {
    processFunction =
        !incomingDspConnections[0].empty() ? &processSignal : &processScalar;
}

string DspOsc::toString() {
    char str[snprintf(NULL, 0, "%s %g", getObjectLabel(), frequency) + 1];
    snprintf(str, sizeof(str), "%s %g", getObjectLabel(), frequency);
    return string(str);
}

void DspOsc::processMessage(int inletIndex, PdMessage *message) {
    switch (inletIndex) {
    case 0: { // update the frequency
        if (message->isFloat(0)) {
            frequency = fabsf(message->getFloat(0));
        }
        break;
    }
    case 1: { // update the phase
        // TODO
        break;
    }
    default:
        break;
    }
}

void DspOsc::processSignal(DspObject *dspObject, int fromIndex, int toIndex) {
    DspOsc *d = reinterpret_cast<DspOsc *>(dspObject);
    float tableSizeFloat = (float)(COS_TABLE_SIZE - 1);
    float multiplier = tableSizeFloat / d->graph->getSampleRate();
    float *input = d->dspBufferAtInlet[0];
    float *output = d->dspBufferAtOutlet[0];
    float phase = d->phase;

    for (int i = fromIndex; i < toIndex; i++) {
        unsigned int lower = (unsigned int)phase;
        float fraction = phase - lower;
        float out = fraction * cos_table[lower] +
                    (1.0f - fraction) * cos_table[lower + 1];
        output[i] = out;
        phase = fmod(phase + input[i] * multiplier, tableSizeFloat);
    }

    if (isnan(phase)) {
        phase = 0.0f;
    }

    d->phase = phase;
}

void DspOsc::processScalar(DspObject *dspObject, int fromIndex, int toIndex) {
    DspOsc *d = reinterpret_cast<DspOsc *>(dspObject);
    float tableSizeFloat = (float)(COS_TABLE_SIZE - 1);
    float multiplier = tableSizeFloat / d->graph->getSampleRate();
    float phase = d->phase;
    float addend = multiplier * d->frequency;
    float *output = d->dspBufferAtOutlet[0];

    for (int i = fromIndex; i < toIndex; i++) {
        unsigned int lower = (unsigned int)phase;
        float fraction = phase - lower;
        float out = fraction * cos_table[lower] +
                    (1.0f - fraction) * cos_table[lower + 1];
        output[i] = out;
        phase = fmod(phase + addend, tableSizeFloat);
    }

    d->phase = phase;
}
