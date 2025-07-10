use std::{hint::black_box, marker::PhantomData};

use abi_stable::{
    external_types::RMutex,
    pointer_trait::ImmutableRef,
    std_types::{RArc, RVec},
};

use crate::{
    event::Event,
    event_hooks::{EventHooks, EventHooksInternal},
    event_ref::{EventRef, EventRefInternal},
    from_void,
    internal_pluggie_context::{InternalEventSender, InternalPluggieCtx},
    to_void,
};

#[derive(Clone)]
pub struct PluggieCtx(RArc<RMutex<InternalPluggieCtx>>);

unsafe impl Send for PluggieCtx {}
unsafe impl Sync for PluggieCtx {}

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
    pub fn subscribe<'a, T: Event + Clone, F: Fn(EventRef<T>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) {
        self.subscribe_with_priority(f, 0.0);
    }
    pub fn subscribe_with_priority<
        'a,
        T: Event + Clone,
        F: Fn(EventRef<T>) + Send + Sync + 'static,
    >(
        &self,
        f: F,
        priority: f32,
    ) {
        let mut lock = self.0.lock();
        let ctx = self.clone();
        lock.subscribe::<T>(
            Box::new(move |ptr| {
                let ctx = ctx.clone();
                let event_ref = unsafe { from_void::<EventRefInternal<T>>(ptr) };
                // println!("Received event: {:p}", event_ref);
                black_box(event_ref.to_raw_ptr());
                let event_ref = EventRef {
                    internal: event_ref,
                    ctx,
                };
                f(black_box(event_ref));
            }),
            priority,
        );
    }
}

impl<T: Event + Clone> EventSender<T> {
    pub fn call(&self, event: &T) {
        let r = EventRefInternal {
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
