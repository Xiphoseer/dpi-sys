#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("dpi-sys/include/context.h");

        type DPI_Context;

        pub fn new_DPI_Context() -> UniquePtr<DPI_Context>;
    }

    unsafe extern "C++" {
        include!("dpi-sys/include/init.h");

        pub fn DPI_Init(ctx: Pin<&mut DPI_Context>) -> i32;
    }

    unsafe extern "C++" {
        include!("dpi-sys/include/create_buffer.h");

        pub fn DPI_Create_buffer(
            name: &CxxString,
            node_id: u64,
            context: Pin<&mut DPI_Context>,
            segments_per_appender: usize,
            participating_appenders: usize,
        ) -> i32;
    }

    unsafe extern "C++" {
        include!("dpi-sys/include/config.h");

        pub fn config_set_rdma_device(val: u32);
        pub fn config_set_rdma_memsize(val: usize);
        pub fn config_set_rdma_numa_region(val: u32);
        pub fn config_set_rdma_ib_port(val: u32);
        pub fn config_set_rdma_gid_idx(val: u8);
        pub fn config_set_dpi_segment_size(val: u32);
        pub fn config_get_dpi_segment_header_size() -> u32;
        pub fn config_set_dpi_internal_buffer_size(val: u32);
        pub fn config_set_dpi_registry_server(val: &CxxString);
        pub fn config_set_dpi_registry_port(val: u32);
        pub fn config_set_dpi_node_port(val: u32);
        pub fn config_dpi_nodes_clear();
        pub fn config_dpi_nodes_push_back(val: &CxxString);
    }

    unsafe extern "C++" {
        include!("dpi-sys/include/server.h");

        type RegistryServer;
        type NodeServer;

        pub fn startServer(self: Pin<&mut NodeServer>) -> bool;
        pub fn startServer(self: Pin<&mut RegistryServer>) -> bool;

        pub fn stopServer(self: Pin<&mut NodeServer>);
        pub fn stopServer(self: Pin<&mut RegistryServer>);

        pub fn new_registry_server() -> UniquePtr<RegistryServer>;
        pub fn new_node_server() -> UniquePtr<NodeServer>;
    }
}
