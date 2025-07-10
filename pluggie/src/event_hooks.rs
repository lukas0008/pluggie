use std::{ffi::c_void, marker::PhantomData};

use abi_stable::{
    StableAbi,
    external_types::RMutex,
    std_types::{RBox, RVec},
};

use crate::{AllLoadedEvent, event::Event};

#[derive(StableAbi)]
#[repr(C)]
pub(crate) struct EventHooksInternal {
    pub(crate) post_event_hooks: RVec<RBox<*const c_void>>,
}

#[derive(StableAbi)]
#[repr(C)]
pub struct EventHooks<T: Event> {
    pub(crate) internal: RMutex<EventHooksInternal>,
    pub(crate) event_type: PhantomData<T>,
}

impl<T: Event> EventHooks<T> {
    // pub fn new() -> Self {}

    pub fn add_hook(&self, hook: Box<dyn Fn(&T)>) {
        let mut lock = self.internal.lock();
        // let raw = Box::into_raw(hook);
        let aaaa = Box::leak(hook);
        unsafe {
            let box_box: RBox<*const c_void> = std::mem::transmute(RBox::new(aaaa));
            lock.post_event_hooks.push(box_box);
        }
    }
    pub fn post_event_hooks<'a>(&'a self) -> Vec<&'a dyn Fn(&T)> {
        let mut lock = self.internal.lock();
        let mut hooks = Vec::<&'a dyn Fn(&T)>::new();
        for hook in lock.post_event_hooks.drain(..) {
            // let hook =
            // unsafe { Box::from_raw(*(hook as *const c_void as *mut *mut dyn Fn(&T))) };
            unsafe {
                let box_box: RBox<&dyn Fn(&T)> = std::mem::transmute(hook);
                // let inner = *hook;
                // let raw: *mut *mut dyn Fn(&T) = std::mem::transmute(*hook);
                // let closure_ptr = raw;
                // let closure_ref = &closure_ptr;
                // println!("hook gotten: {:p}", box_box);
                hooks.push(*box_box);
            }
        }
        hooks
    }
}

impl Drop for EventHooksInternal {
    fn drop(&mut self) {
        for _hook in self.post_event_hooks.drain(..) {
            // TODO: properly drop the hook, cause rn there is a memory leak
        }
    }
}
