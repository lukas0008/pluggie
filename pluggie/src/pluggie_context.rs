use std::{hint::black_box, marker::PhantomData};

use abi_stable::{
    external_types::RMutex,
    pointer_trait::ImmutableRef,
    std_types::{RArc, RVec},
};

use crate::{
    event::Event,
    event_hooks::{EventHooks, EventHooksInternal},
    event_ref::EventRef,
    from_void,
    internal_pluggie_context::{InternalEventSender, InternalPluggieCtx},
    to_void,
};

#[derive(Clone)]
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
    pub fn subscribe<'a, T: Event + Clone, F: Fn(&EventRef<T>) + 'static>(&self, f: F) {
        let mut lock = self.0.lock();
        lock.subscribe::<T>(Box::new(move |ptr| {
            let event_ref = unsafe { from_void::<EventRef<T>>(ptr) };
            // println!("Received event: {:p}", event_ref);
            black_box(event_ref.to_raw_ptr());
            f(black_box(event_ref));
        }));
    }
}

impl<T: Event + Clone> EventSender<T> {
    pub fn call(&self, event: &T) {
        let r = EventRef {
            event,
            hooks: RArc::new(EventHooks {
                event_type: PhantomData,
                internal: RMutex::new(EventHooksInternal {
                    post_event_hooks: RVec::new(),
                }),
            }),
        };
        let r_ptr = &r;
        // println!("Passing event: {:p}", r_ptr);
        self.0.call(unsafe { to_void(r_ptr) });
        let hooks = r.hooks.post_event_hooks();
        for hook in hooks.into_iter() {
            hook(event);
            // Box::leak(hook);
        }
    }
}
