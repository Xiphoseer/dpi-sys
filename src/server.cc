#include "dpi-sys/include/server.h"

std::unique_ptr<RegistryServer> new_registry_server()
{
    return std::unique_ptr<RegistryServer>(new RegistryServer());
}

std::unique_ptr<NodeServer> new_node_server()
{
    return std::unique_ptr<NodeServer>(new NodeServer());
}
