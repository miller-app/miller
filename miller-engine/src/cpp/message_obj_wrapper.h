#ifndef _MESSAGE_OBJ_WRAPPER_H_
#define _MESSAGE_OBJ_WRAPPER_H_

#include <stdio.h>

#include "MessageObject.h"

struct MessageObjAdapter;

class MessageObjWrapper : public MessageObject {
  public:
    MessageObjWrapper(int numMessageInlets, int numMessageOutlets,
                      PdGraph *graph, MessageObjAdapter *adapter);

    ~MessageObjWrapper();

    void receiveMessage(int inletIndex, PdMessage *message);

    void processMessage(int inletIndex, PdMessage *message);

    void sendMessage(int outletIndex, PdMessage *message);

    ConnectionType getConnectionType(int outletIndex);

    list<ObjectLetPair> getIncomingConnections(unsigned int inletIndex);

    list<ObjectLetPair> getOutgoingConnections(unsigned int outletIndex);

    void addConnectionFromObjectToInlet(MessageObject *messageObject,
                                        int outletIndex, int inletIndex);

    void addConnectionToObjectFromOutlet(MessageObject *messageObject,
                                         int inletIndex, int outletIndex);

    void removeConnectionFromObjectToInlet(MessageObject *messageObject,
                                           int outletIndex, int inletIndex);

    void removeConnectionToObjectFromOutlet(MessageObject *messageObject,
                                            int inletIndex, int outletIndex);

    void updateOutgoingMessageConnection(MessageObject *messageObject,
                                         int oldInletIndex, int outletIndex,
                                         int newInletIndex);

    void updateIncomingMessageConnection(MessageObject *messageObject,
                                         int oldOutletIndex, int inletIndex,
                                         int newOutletIndex);

    static const char *getObjectLabel();
    string toString();

    ObjectType getObjectType();

    bool doesProcessAudio();

    bool shouldDistributeMessageToInlets();

    bool isLeafNode();

    list<DspObject *> getProcessOrder();

    void resetOrderedFlag();

    unsigned int getNumInlets();
    unsigned int getNumOutlets();

    PdGraph *getGraph();

    void getCanvasPosition(float *x, float *y);

    void setCanvasPosition(float x, float y);

  private:
    MessageObjAdapter *adapter;
};

// Rust FFI

extern "C" void message_obj_receive_message(MessageObjAdapter *adapter,
                                            size_t inlet, PdMessage *message);

extern "C" void message_obj_process_message(MessageObjAdapter *adapter,
                                            size_t inlet, PdMessage *message);

extern "C" void message_obj_send_message(MessageObjAdapter *adapter,
                                         size_t outlet, PdMessage *message);

#endif // _MESSAGE_OBJ_WRAPPER_H_
