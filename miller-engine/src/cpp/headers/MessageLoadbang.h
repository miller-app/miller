/*
 *  Copyright 2009 Reality Jockey, Ltd.
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

#ifndef _MESSAGE_LOADBANG_H_
#define _MESSAGE_LOADBANG_H_

#include "MessageObject.h"

/** [loadbang] */
class MessageLoadbang : public MessageObject {

  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    MessageLoadbang(PdGraph *graph);
    ~MessageLoadbang();

    static const char *getObjectLabel();
    std::string toString();
};

inline const char *MessageLoadbang::getObjectLabel() { return "loadbang"; }

inline std::string MessageLoadbang::toString() {
    return MessageLoadbang::getObjectLabel();
}

#endif // _MESSAGE_LOADBANG_H_
