/*
 *  Copyright 2010 Reality Jockey, Ltd.
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

#ifndef _MESSAGE_LIST_LENGTH_H_
#define _MESSAGE_LIST_LENGTH_H_

#include "MessageObject.h"

/** [list length] */
class MessageListLength : public MessageObject {

  public:
    MessageListLength(PdMessage *initMessage, PdGraph *graph);
    ~MessageListLength();

    static const char *getObjectLabel();
    std::string toString();

  private:
    void processMessage(int inletIndex, PdMessage *message);
};

inline const char *MessageListLength::getObjectLabel() { return "list length"; }

inline std::string MessageListLength::toString() {
    return MessageListLength::getObjectLabel();
}

#endif // _MESSAGE_LIST_LENGTH_H_
