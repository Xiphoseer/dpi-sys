extern crate ibverbs_sys;
extern crate numa_sys;
extern crate protobuf_sys;
extern crate zmq_sys;

include!("ffi.rs");

use std::num::NonZeroI32;

use cxx::{CxxString, UniquePtr};

#[derive(Debug)]
pub struct Error(NonZeroI32);

impl Error {
    fn result(h: i32) -> Result<(), Self> {
        match NonZeroI32::new(h) {
            Some(err_code) => Err(Self(err_code)),
            None => Ok(()),
        }
    }
}

pub struct Context(UniquePtr<ffi::DPI_Context>);

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self(ffi::new_DPI_Context())
    }

    pub fn init(&mut self) {
        ffi::DPI_Init(self.0.pin_mut());
    }

    pub fn create_buffer(&mut self, name: &CxxString, node_id: u64) -> Result<(), Error> {
        let h = ffi::DPI_Create_buffer(name, node_id, self.0.pin_mut(), 1, 1);
        Error::result(h)
    }
}
