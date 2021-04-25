/*
 *  Copyright 2017 Jacob A. Stern
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

#include "MessagePoly.h"
#include <climits>

MessageObject *MessagePoly::newObject(PdMessage *initMessage, PdGraph *graph) {
    return new MessagePoly(initMessage, graph);
}

MessagePoly::MessagePoly(PdMessage *initMessage, PdGraph *graph)
    : MessageObject(initMessage->getNumElements(), 3, graph), velocity(0.0f),
      serial(0L) {
    n = 1;
    if (initMessage->isFloat(0)) {
        int value = (int)initMessage->getFloat(0);
        if (value > 1) {
            n = value;
        }
    }
    voices = std::vector<Voice>(n);
    steal = false;
    if (initMessage->isFloat(1)) {
        steal = initMessage->getFloat(1) == 1.0f;
    }
}

MessagePoly::~MessagePoly() {}

void MessagePoly::processMessage(int inletIndex, PdMessage *message) {
    switch (inletIndex) {
    case 0:
        if (message->isFloat(0)) {
            if (message->isFloat(1)) {
                velocity = message->getFloat(1);
            }
            // Translated from
            // https://github.com/pure-data/pure-data/blob/master/src/x_midi.c
            float val = message->getFloat(0);
            Voice *firstOn = nullptr;
            Voice *firstOff = nullptr;
            int onIndex = 0;
            int offIndex = 0;
            if (velocity > 0) {
                // Note on, look for a vacant voice
                unsigned long serialOn = ULONG_MAX;
                unsigned long serialOff = ULONG_MAX;
                for (int i = 0; i < n; i++) {
                    Voice *v = &voices[i];
                    if (v->used && v->serial < serialOn) {
                        firstOn = v;
                        serialOn = v->serial;
                        onIndex = i;
                    } else if (!v->used && v->serial < serialOff) {
                        firstOff = v;
                        serialOff = v->serial;
                        offIndex = i;
                    }
                }
                if (firstOff != nullptr) {
                    PdMessage *outgoingMessage = PD_MESSAGE_ON_STACK(1);
                    double timestamp = message->getTimestamp();
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               velocity);
                    sendMessage(2, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp, val);
                    sendMessage(1, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               offIndex + 1);
                    sendMessage(0, outgoingMessage);
                    firstOff->pitch = val;
                    firstOff->used = true;
                    firstOff->serial = serial++;
                } else if (firstOn != nullptr && steal) {
                    // If no available voice, steal one
                    PdMessage *outgoingMessage = PD_MESSAGE_ON_STACK(1);
                    double timestamp = message->getTimestamp();
                    outgoingMessage->initWithTimestampAndFloat(timestamp, 0);
                    sendMessage(2, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               firstOn->pitch);
                    sendMessage(1, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               onIndex + 1);
                    sendMessage(0, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               velocity);
                    sendMessage(2, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp, val);
                    sendMessage(1, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               onIndex + 1);
                    sendMessage(0, outgoingMessage);
                    firstOn->pitch = val;
                    firstOn->serial = serial++;
                }
            } else {
                // Note off, turn off oldest match
                unsigned long serialOn = ULONG_MAX;
                for (int i = 0; i < n; i++) {
                    Voice *v = &voices[i];
                    if (v->used && v->pitch == val && v->serial < serialOn) {
                        firstOn = v;
                        serialOn = v->serial;
                        onIndex = i;
                    }
                }
                if (firstOn != nullptr) {
                    firstOn->used = false;
                    firstOn->serial = serial++;
                    PdMessage *outgoingMessage = PD_MESSAGE_ON_STACK(1);
                    double timestamp = message->getTimestamp();
                    outgoingMessage->initWithTimestampAndFloat(timestamp, 0);
                    sendMessage(2, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               firstOn->pitch);
                    sendMessage(1, outgoingMessage);
                    outgoingMessage->initWithTimestampAndFloat(timestamp,
                                                               onIndex + 1);
                    sendMessage(0, outgoingMessage);
                }
            }
        }
        break;
    case 1:
        if (message->isFloat(0)) {
            velocity = message->getFloat(0);
        } else if (message->isSymbol(0, "stop")) {
            // TODO: implement stop
        } else if (message->isSymbol(0, "clear")) {
            // TODO: implement clear
        }
        break;
    }
}
