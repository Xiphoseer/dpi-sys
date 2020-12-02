#pragma once
#include <string>
#include "../utils/Config.h"

void config_set_rdma_memsize(size_t val);
void config_set_rdma_device(uint32_t val);
void config_set_rdma_numa_region(uint32_t val);
void config_set_rdma_ib_port(uint32_t val);
void config_set_rdma_gid_idx(uint8_t val);
void config_set_dpi_segment_size(uint32_t val);
uint32_t config_get_dpi_segment_header_size();
void config_set_dpi_internal_buffer_size(uint32_t val);
void config_set_dpi_registry_server(const std::string &val);
void config_set_dpi_registry_port(uint32_t val);
void config_set_dpi_node_port(uint32_t val);
void config_dpi_nodes_clear();
void config_dpi_nodes_push_back(const std::string &val);
