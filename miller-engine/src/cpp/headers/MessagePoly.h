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

#ifndef _MESSAGE_POLY_H
#define _MESSAGE_POLY_H

#include "MessageObject.h"
#include <vector>

/** [poly] **/
class MessagePoly : public MessageObject {

  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    MessagePoly(PdMessage *initMessage, PdGraph *graph);
    ~MessagePoly();

    static const char *getObjectLabel();
    std::string toString();

  private:
    void processMessage(int inletIndex, PdMessage *message);

    struct Voice {
        Voice() : pitch(0), used(false), serial(0L) {}

        float pitch;
        bool used;
        unsigned long serial;
    };

    std::vector<Voice> voices;
    int n;
    float velocity;
    unsigned long serial;
    bool steal;
};

inline const char *MessagePoly::getObjectLabel() { return "poly"; }

inline std::string MessagePoly::toString() {
    return MessagePoly::getObjectLabel();
}

#endif // _MESSAGE_POLY_H
