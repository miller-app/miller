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

#ifndef _MESSAGE_LOGICAL_AND_H_
#define _MESSAGE_LOGICAL_AND_H_

#include "MessageObject.h"

/** [&&] */
class MessageLogicalAnd : public MessageObject {

  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    MessageLogicalAnd(PdMessage *initMessage, PdGraph *graph);
    ~MessageLogicalAnd();

    static const char *getObjectLabel();
    std::string toString();

  private:
    void processMessage(int inletIndex, PdMessage *message);

    float left;
    float right;
};

inline const char *MessageLogicalAnd::getObjectLabel() { return "&&"; }

inline std::string MessageLogicalAnd::toString() {
    return MessageLogicalAnd::getObjectLabel();
}

#endif // _MESSAGE_LOGICAL_AND_H_
