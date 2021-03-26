/*
 *  Copyright 2017 Jacob Stern
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

#ifndef _DSP_TABLE_WRITE_H_
#define _DSP_TABLE_WRITE_H_

#include "DspObject.h"
#include "TableReceiverInterface.h"

/* [tabwrite~ name] */
class DspTableWrite : public DspObject, public TableReceiverInterface {
  
  public:
    static MessageObject *newObject(PdMessage *initMessage, PdGraph *graph);
    DspTableWrite(PdMessage *initMessage, PdGraph *graph);
    ~DspTableWrite();
    
    static const char *getObjectLabel();
    std::string toString();
    ObjectType getObjectType();
  
    char *getName();
    void setTable(MessageTable *table);
    
  private:
    void processMessage(int inletIndex, PdMessage *message);
    void processDspWithIndex(int fromIndex, int toIndex);

    int index;
    bool stopped;
    char *name;
    MessageTable *table;
};

inline std::string DspTableWrite::toString() {
  return DspTableWrite::getObjectLabel();
}

inline const char *DspTableWrite::getObjectLabel() {
  return "tabwrite~";
}

inline ObjectType DspTableWrite::getObjectType() {
  return DSP_TABLE_WRITE;
}

inline char *DspTableWrite::getName() {
  return name;
}

#endif // _DSP_TABLE_WRITE_H_
