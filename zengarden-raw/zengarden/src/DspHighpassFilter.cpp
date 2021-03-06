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

#include "DspHighpassFilter.h"
#include "PdGraph.h"

MessageObject *DspHighpassFilter::newObject(PdMessage *initMessage,
                                            PdGraph *graph) {
    return new DspHighpassFilter(initMessage, graph);
}

DspHighpassFilter::DspHighpassFilter(PdMessage *initMessage, PdGraph *graph)
    : DspFilter(2, graph) {
    // by default, the filter is initialised completely open
    calcFiltCoeff(initMessage->isFloat(0) ? initMessage->getFloat(0) : 0.0f);
}

DspHighpassFilter::~DspHighpassFilter() {
    // nothing to do
}

// http://en.wikipedia.org/wiki/High-pass_filter
void DspHighpassFilter::calcFiltCoeff(float fc) {
    if (fc > 0.5f * graph->getSampleRate())
        fc = 0.5f * graph->getSampleRate();
    else if (fc < 0.0f)
        fc = 10.0f;

    float alpha =
        graph->getSampleRate() / ((2.0f * M_PI * fc) + graph->getSampleRate());
    b[0] = alpha;
    b[1] = -alpha;
    b[2] = 0.0f;
    b[3] = -alpha;
    b[4] = 0.0f;
}

void DspHighpassFilter::processMessage(int inletIndex, PdMessage *message) {
    switch (inletIndex) {
    case 0: {
        switch (message->getType(0)) {
        case FLOAT: {
            //          signalConstant = message->getFloat(0);
            break;
        }
        case SYMBOL: {
            if (message->isSymbol(0, "clear")) {
                x1 = x2 = 0.0f;
                dspBufferAtOutlet[0][0] = dspBufferAtOutlet[0][1] = 0.0f;
            }
            break;
        }
        default:
            break;
        }
        break;
    }
    case 1: {
        if (message->isFloat(0)) {
            calcFiltCoeff(message->getFloat(0));
        }
        break;
    }
    default:
        break;
    }
}
