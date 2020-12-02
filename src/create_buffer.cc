#include "dpi-sys/include/create_buffer.h"

int DPI_Create_buffer(const std::string &name, NodeID node_id, DPI_Context &context, size_t segmentsPerAppender, size_t participatingAppenders)
{
    //Check that DPI_Init has been called
    if (context.registry_client == nullptr)
        return DPI_NOT_INITIALIZED;

    context.registry_client->registerBuffer(new BufferHandle(name, node_id, segmentsPerAppender, participatingAppenders, Config::DPI_SEGMENT_SIZE - sizeof(Config::DPI_SEGMENT_HEADER_t)));
    BufferHandle *buffHandle = context.registry_client->retrieveBuffer(name);
    context.buffer_writers[name] = new BufferWriterBW(name, context.registry_client, Config::DPI_INTERNAL_BUFFER_SIZE);

    context.buffer_readers[name] = new BufferReader(buffHandle, context.registry_client);
    return DPI_SUCCESS;
}
