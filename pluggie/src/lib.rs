use crate::event::Event;
use abi_stable::StableAbi;
use std::ffi::c_void;

pub mod event;
pub mod internal_pluggie_context;
pub mod pluggie_context;
pub mod plugin;
pub mod reexports {
    pub use sha2_const;
}

pub const VERSION: u32 = 0;

#[macro_export]
macro_rules! event_name {
    ($name: expr) => {
        pluggie::reexports::sha2_const::Sha256::new()
            .update($name.as_bytes())
            .finalize()
    };
}

#[derive(StableAbi, Copy, Clone)]
#[repr(transparent)]
pub struct AllLoadedEvent;

pub unsafe fn to_void<'a, T: Sized>(value: &'a T) -> usize {
    unsafe { std::mem::transmute(value) }
}

pub unsafe fn from_void<'a, T: Sized>(ptr: usize) -> &'a T {
    unsafe { std::mem::transmute(ptr) }
}

impl Event for AllLoadedEvent {
    const NAME: [u8; 32] = sha2_const::Sha256::new().update(b"all_loaded").finalize();
}
