use std::{ffi::c_void, fmt::Debug, hint::black_box, marker::PhantomData};

use crate::{event::Event, exposable::Exposable, to_void};
use abi_stable::{
    StableAbi,
    external_types::RRwLock,
    std_types::{RArc, RHashMap, RVec},
};

#[derive(StableAbi)]
#[repr(C)]
pub struct ChannelFunc {
    func: *mut c_void,
    priority: f32,
}

#[repr(C)]
pub struct InternalPluggieCtx {
    // channels_subscribes: RHashMap<[u8; 32], RReceiver<usize>>,
    channel_calls: RHashMap<[u8; 32], RArc<RRwLock<RVec<ChannelFunc>>>>,
    exposed: RHashMap<[u8; 32], usize>,
}

pub struct InternalEventSender<T: Event> {
    // sender: crossbeam_channel::RSender<usize>,
    // callers: RwLock<Vec<Box<dyn Fn(T)>>>,
    pub(crate) callers: RArc<RRwLock<RVec<ChannelFunc>>>,
    pub(crate) event_type: PhantomData<T>,
}

impl<T: Event> InternalEventSender<T> {
    pub(crate) fn call(&self, data: usize) {
        let lock = self.callers.read();
        for caller in lock.iter() {
            let caller = &caller.func;
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
            exposed: RHashMap::new(),
        }
    }

    pub(crate) fn register_event<T: Event>(&mut self) -> InternalEventSender<T> {
        let entry = self.channel_calls.entry(T::NAME_HASH);
        let entry = entry.or_insert_with(|| {
            let callers = RArc::new(RRwLock::new(RVec::new()));
            callers
        });
        InternalEventSender {
            // sender,
            callers: entry.clone(),
            event_type: PhantomData,
        }
    }

    pub(crate) fn subscribe<T: Event>(&mut self, closure: Box<dyn Fn(usize)>, priority: f32) {
        // TODO: fix the memory leak here
        let raw = Box::into_raw(closure);
        let void_ptr = {
            let boxed_raw = Box::new(raw);
            let void_ptr: *mut c_void = Box::into_raw(boxed_raw) as *mut c_void;
            void_ptr
        };

        let calls = self.channel_calls.entry(T::NAME_HASH);
        let calls = calls.or_insert_with(|| {
            let callers = RArc::new(RRwLock::new(RVec::new()));
            callers
        });
        let mut calls = calls.write();
        calls.push(ChannelFunc {
            func: void_ptr,
            priority,
        });
        calls.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        // println!();
        // let receiver = self.channels_subscribes.get(&T::NAME).unwrap();
        // InternalEventReceiver {
        //     receiver: receiver.clone(),
        //     event_type: PhantomData,
        //
        // }
    }

    pub(crate) fn expose<T: Exposable>(&mut self, value: T) {
        let boxed = Box::new(value);
        let ptr = Box::leak(boxed);
        self.exposed.insert(T::NAME_HASH, unsafe { to_void(ptr) });
    }

    pub(crate) fn get<T: Exposable + Debug>(&self) -> Option<T> {
        let opt = self.exposed.get(&T::NAME_HASH);
        // unsafe { std::mem::transmute::<_, Option<&T>>(opt) }.map(|boxed| (*boxed).clone())
        let opt = opt.map(|ptr| (unsafe { black_box(std::mem::transmute::<_, &T>(*ptr)) }));

        // if let Some(ptr) = opt {
        //     println!("gurt {:p}", ptr);
        // }
        // dbg!(&opt);

        opt.cloned()
    }
}
