#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]

use std::sync::Arc;

use crate::{event::Event, plugin::PluginRef};

pub mod curry;
pub mod event;
pub mod event_hooks;
pub mod event_ref;
pub mod exposable;
pub mod internal_pluggie_context;
pub mod pluggie_context;
pub mod plugin;
pub mod reexports {
    pub use abi_stable;
    pub use sha2_const;
}

pub const VERSION: u32 = 0;

#[macro_export]
macro_rules! name_hash {
    ($name: expr) => {
        pluggie::reexports::sha2_const::Sha256::new()
            .update($name.as_bytes())
            .finalize()
    };
}

#[derive(Clone)]
#[repr(transparent)]
pub struct AllLoadedEvent {
    pub plugins: Vec<Arc<PluginRef>>,
}

pub unsafe fn to_void<'a, T: Sized>(value: &'a T) -> usize {
    unsafe { std::mem::transmute(value) }
}

pub unsafe fn from_void<'a, T: Sized>(ptr: usize) -> &'a T {
    unsafe { std::mem::transmute(ptr) }
}

impl Event for AllLoadedEvent {
    const NAME: &'static str = "pluggie:all_loaded";
    // const NAME_HASH: [u8; 32] = sha2_const::Sha256::new().update(b"all_loaded").finalize();
}
