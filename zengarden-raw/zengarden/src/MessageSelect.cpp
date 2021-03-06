/*
 *  Copyright 2009,2010,2011 Reality Jockey, Ltd.
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

#include "MessageSelect.h"
#include "PdGraph.h"

MessageObject *MessageSelect::newObject(PdMessage *initMessage,
                                        PdGraph *graph) {
    return new MessageSelect(initMessage, graph);
}

MessageSelect::MessageSelect(PdMessage *initMessage, PdGraph *graph)
    : MessageObject((initMessage->getNumElements() < 2) ? 2 : 1,
                    (initMessage->getNumElements() < 2)
                        ? 2
                        : initMessage->getNumElements() + 1,
                    graph) {
    selectorMessage = initMessage->copyToHeap();
}

MessageSelect::~MessageSelect() { selectorMessage->freeMessage(); }

void MessageSelect::processMessage(int inletIndex, PdMessage *message) {
    switch (inletIndex) {
    case 0: {
        MessageAtom *messageElement = message->getElement(0);
        int numSelectors = selectorMessage->getNumElements();
        for (int i = 0; i < numSelectors; i++) {
            if (selectorMessage->atomIsEqualTo(i, messageElement)) {
                // send bang from matching outlet
                PdMessage *outgoingMessage = PD_MESSAGE_ON_STACK(1);
                outgoingMessage->initWithTimestampAndBang(
                    message->getTimestamp());
                sendMessage(i, outgoingMessage);
                return;
            }
        }

        // message does not match any selector. Send it out to of the last
        // outlet.
        sendMessage(numSelectors, message);
        break;
    }
    case 1: {
        // TODO(mhroth): be able to set the selector
        graph->printErr("select currently does not support setting the "
                        "selector via the right inlet.\n");
        break;
    }
    default: {
        break;
    }
    }
}
