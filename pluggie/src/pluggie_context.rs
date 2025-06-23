use abi_stable::{external_types::RMutex, std_types::RArc};

use crate::{
    event::Event,
    from_void,
    internal_pluggie_context::{InternalEventSender, InternalPluggieCtx},
    to_void,
};

pub struct PluggieCtx(RArc<RMutex<InternalPluggieCtx>>);
pub struct EventSender<T: Event>(InternalEventSender<T>);

impl PluggieCtx {
    pub fn new(internal: RArc<RMutex<InternalPluggieCtx>>) -> Self {
        Self(internal)
    }
    pub fn register_event<T: Event>(&self) -> EventSender<T> {
        let mut lock = self.0.lock();
        let sender = lock.register_event::<T>();
        EventSender(sender)
    }
    pub fn subscribe<T: Event, F: Fn(&T) + 'static>(&self, f: F) {
        let mut lock = self.0.lock();
        lock.subscribe::<T>(Box::new(move |ptr| {
            let event: &T = unsafe { from_void(ptr) };
            f(&event);
        }));
    }
}

impl<T: Event> EventSender<T> {
    pub fn call(&self, event: &T) {
        self.0.call(unsafe { to_void(event) });
    }
}
