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

#ifndef _MESSAGE_METRO_H_
#define _MESSAGE_METRO_H_

#include "MessageObject.h"

class PdGraph;

/** [metro], [metro float] */
class MessageMetro : public MessageObject {

  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    MessageMetro(PdMessage *initMessage, PdGraph *graph);
    ~MessageMetro();

    static const char *getObjectLabel();
    std::string toString();

    void sendMessage(int outletIndex, PdMessage *message);

  private:
    void processMessage(int inletIndex, PdMessage *message);

    /* Cancels the current <code>pendingMessage</code>. */
    void stopMetro();

    /** @param  timestamp The time at which the metro should be started. */
    void startMetro(double timestamp);

    PdMessage *pendingMessage;
    double intervalInMs;
};

inline const char *MessageMetro::getObjectLabel() { return "metro"; }

#endif // _MESSAGE_METRO_H_
