#include "dpi-sys/include/config.h"

void config_set_rdma_memsize(size_t val)
{
    Config::RDMA_MEMSIZE = val;
}

void config_set_rdma_device(uint32_t val)
{
    Config::RDMA_DEVICE = val;
}

void config_set_rdma_numa_region(uint32_t val)
{
    Config::RDMA_NUMAREGION = val;
}

void config_set_rdma_ib_port(uint32_t val)
{
    Config::RDMA_IBPORT = val;
}

void config_set_rdma_gid_idx(uint8_t val)
{
    Config::RDMA_GID_IDX = val;
}

void config_set_dpi_segment_size(uint32_t val)
{
    Config::DPI_SEGMENT_SIZE = val;
}

uint32_t config_get_dpi_segment_header_size()
{
    return sizeof(Config::DPI_SEGMENT_HEADER_t);
}

void config_set_dpi_internal_buffer_size(uint32_t val)
{
    Config::DPI_INTERNAL_BUFFER_SIZE = val;
}

void config_set_dpi_registry_server(const std::string &val)
{
    Config::DPI_REGISTRY_SERVER = val;
}

void config_set_dpi_registry_port(uint32_t val)
{
    Config::DPI_REGISTRY_PORT = val;
}

void config_set_dpi_node_port(uint32_t val)
{
    Config::DPI_NODE_PORT = val;
}

void config_dpi_nodes_clear()
{
    Config::DPI_NODES.clear();
}

void config_dpi_nodes_push_back(const std::string &val)
{
    Config::DPI_NODES.push_back(val);
}
