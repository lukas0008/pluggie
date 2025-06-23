use std::{
    ffi::c_void,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use abi_stable::{
    DynTrait, RRef, StableAbi,
    external_types::{
        RRwLock,
        crossbeam_channel::{self, RReceiver, RSender},
    },
    reexports::SelfOps,
    std_types::{RArc, RBox, RHashMap, RVec},
};

use crate::event::Event;

#[derive(StableAbi)]
#[repr(C)]
pub struct InternalPluggieCtx {
    // channels_subscribes: RHashMap<[u8; 32], RReceiver<usize>>,
    channel_calls: RHashMap<[u8; 32], RArc<RRwLock<RVec<*mut c_void>>>>,
}

pub struct InternalEventSender<T: Event> {
    // sender: crossbeam_channel::RSender<usize>,
    // callers: RwLock<Vec<Box<dyn Fn(T)>>>,
    callers: RArc<RRwLock<RVec<*mut c_void>>>,
    event_type: PhantomData<T>,
}

impl<T: Event> InternalEventSender<T> {
    pub(crate) fn call(&self, data: usize) {
        let lock = self.callers.read();
        for caller in lock.iter() {
            unsafe {
                let raw: *mut *mut dyn Fn(usize) = std::mem::transmute(*caller);
                let closure_ptr = *raw;
                let closure_ref = &*closure_ptr;
                closure_ref(data);
            }
        }
    }
}

impl InternalPluggieCtx {
    pub fn new() -> Self {
        Self {
            // channels_subscribes: RHashMap::new(),
            channel_calls: RHashMap::new(),
        }
    }

    pub(crate) fn register_event<T: Event>(&mut self) -> InternalEventSender<T> {
        // let (sender, receiver) = crossbeam_channel::unbounded();
        // self.channels_subscribes.insert(T::NAME, receiver);
        let callers = RArc::new(RRwLock::new(RVec::new()));
        self.channel_calls.insert(T::NAME, callers.clone());
        InternalEventSender {
            // sender,
            callers,
            event_type: PhantomData,
        }
    }

    pub(crate) fn subscribe<T: Event>(&mut self, closure: Box<dyn Fn(usize)>) {
        // TODO: fix the memory leak here
        let raw = Box::into_raw(closure);
        let void_ptr = {
            let boxed_raw = Box::new(raw);
            let void_ptr: *mut c_void = Box::into_raw(boxed_raw) as *mut c_void;
            void_ptr
        };

        let calls = self.channel_calls.entry(T::NAME);
        let calls = calls.get().expect("Event was not registered");
        let mut calls = calls.write();
        calls.push(void_ptr);
        // println!();
        // let receiver = self.channels_subscribes.get(&T::NAME).unwrap();
        // InternalEventReceiver {
        //     receiver: receiver.clone(),
        //     event_type: PhantomData,
        //
        // }
    }
}
