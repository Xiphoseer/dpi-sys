use cxx::{UniquePtr, let_cxx_string};

use dpi_sys::*;

type State = (UniquePtr<ffi::NodeServer>, UniquePtr<ffi::RegistryServer>);

fn setup() -> State {
    ffi::config_set_rdma_device(0);
    //ffi::config_set_rdma_ib_port(1);
    ffi::config_set_rdma_gid_idx(2);
    ffi::config_set_rdma_memsize(1024usize * 1024 * 1024);  //1GB
    ffi::config_set_rdma_numa_region(0);
    ffi::config_set_dpi_segment_size(2048 + ffi::config_get_dpi_segment_header_size());
    ffi::config_set_dpi_internal_buffer_size(1024);
    let_cxx_string!(registry_server = "127.0.0.1:5400");
    ffi::config_set_dpi_registry_server(&registry_server);
    ffi::config_set_dpi_registry_port(5300);
    ffi::config_set_dpi_node_port(5400);
    ffi::config_dpi_nodes_clear();
    let_cxx_string!(dpi_test_node = "127.0.0.1:5400");
    ffi::config_dpi_nodes_push_back(&dpi_test_node);

    println!("Starting NodeServer");
    let mut node_server = ffi::new_node_server();
    assert!(node_server.pin_mut().startServer());
    println!("NodeServer started");

    println!("Starting Registry Server");
    let mut reg_server = ffi::new_registry_server();
    assert!(reg_server.pin_mut().startServer());
    println!("Registry Server started");

    (node_server, reg_server)
}

fn teardown((mut node_server, mut reg_server): State) {
    node_server.pin_mut().stopServer();
    reg_server.pin_mut().stopServer();
}

fn main() {
    let state = setup();
    println!("Setup complete!");

    let_cxx_string!(buffer_name = "buffer");
    let _data1: &[u8] = b"Hello ";
    let _data2: &[u8] = b"World!";
    let rcv_node_id: u64 = 1; //ID is mapped to a concrete node in cluster spec

    let mut context = Context::new();
    println!("Initializing context");
    context.init();

    println!("Creating buffer");
    context.create_buffer(&buffer_name, rcv_node_id).unwrap();
    // DPI_Append(buffer_name, (void*)data1, sizeof(data1), context);
    // DPI_Append(buffer_name, (void*)data2, sizeof(data2), context);
    // DPI_Close_buffer(buffer_name, context);

    // size_t buffer_size = 0;
    // void* buffer_ptr = nullptr;
    // DPI_Get_buffer(buffer_name, buffer_size, buffer_ptr, context);

    //for(size_t i = 0; i < buffer_size; i++)
    //{
    //      cout << ((char*)buffer_ptr)[i];
    //}
    //DPI_Finalize(context);

    println!("Done, tearing down!");
    teardown(state);
}