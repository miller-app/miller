#include "message_obj_wrapper.h"

MessageObjWrapper::MessageObjWrapper(int numMessageInlets,
                                     int numMessageOutlets, PdGraph *graph_,
                                     MessageObjAdapter *adapter_)
    : MessageObject(numMessageInlets, numMessageOutlets, graph_) {
    graph = graph_;
    adapter = adapter_;
}

MessageObjWrapper::~MessageObjWrapper() {
    // TODO release the adapter - needs the distructor implemented on the Rust's
    // side (to free Box using Box::from_raw)
}

void MessageObjWrapper::receiveMessage(int inletIndex, PdMessage *message) {
    message_obj_receive_message(adapter, (size_t)inletIndex, message);
}

void MessageObjWrapper::processMessage(int inletIndex, PdMessage *message) {
    message_obj_process_message(adapter, (size_t)inletIndex, message);
}

void MessageObjWrapper::sendMessage(int outletIndex, PdMessage *message) {
    message_obj_send_message(adapter, (size_t)outletIndex, message);
}
