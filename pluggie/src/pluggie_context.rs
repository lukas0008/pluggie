use std::{collections::HashMap, fmt::Debug, hint::black_box, marker::PhantomData, sync::Arc};

use abi_stable::{
    external_types::RMutex,
    pointer_trait::ImmutableRef,
    std_types::{RArc, RVec},
};

use crate::{
    event::Event,
    event_hooks::{EventHooks, EventHooksInternal},
    event_ref::{EventRef, EventRefInternal},
    exposable::Exposable,
    from_void,
    internal_pluggie_context::{InternalEventSender, InternalPluggieCtx},
    plugin::{PluginId, PluginRef},
    to_void,
};

#[derive(Clone)]
#[repr(C)]
pub struct PluggieCtx {
    internal: RArc<RMutex<InternalPluggieCtx>>,
    plugin_id: PluginId,
}

unsafe impl Send for PluggieCtx {}

pub struct EventSender<T: Event>(InternalEventSender<T>);
unsafe impl<T: Event> Send for EventSender<T> {}

impl PluggieCtx {
    pub fn clone_with_plugin_id(&self, new_id: PluginId) -> Self {
        let mut new = self.clone();
        new.plugin_id = new_id;
        new
    }
    pub fn new(internal: RArc<RMutex<InternalPluggieCtx>>, plugin_id: PluginId) -> Self {
        Self {
            internal,
            plugin_id,
        }
    }
    pub fn register_event<T: Event>(&self) -> EventSender<T> {
        let mut lock = self.internal.lock();
        let sender = lock.register_event::<T>();
        EventSender(sender)
    }
    pub fn subscribe<'a, T: Event, F: Fn(EventRef<T>) + Send + Sync + 'static>(&self, f: F) {
        self.subscribe_with_priority(f, 0.0);
    }
    pub fn subscribe_with_priority<'a, T: Event, F: Fn(EventRef<T>) + Send + Sync + 'static>(
        &self,
        f: F,
        priority: f32,
    ) {
        let mut lock = self.internal.lock();
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
            self.plugin_id,
        );
    }
    /// Exposes a value so that it can be accessed by other plugins.
    ///
    /// The value type **SHOULD NOT HAVE ANY METHODS AT ALL** the only acceptable methods are those that only call a function, which is a field of the value type.
    ///
    /// For an example of how to expose methods in this way, look at mc-network::NetworkContext
    pub fn expose<T: Exposable>(&self, value: T) {
        let mut lock = self.internal.lock();
        println!("INFO [pluggie]: Exposing {}", T::NAME);
        lock.expose(value);
    }
    pub fn get<T: Exposable + Debug>(&self) -> Option<T> {
        let lock = self.internal.lock();
        lock.get()
    }
    pub fn get_plugin_map(&self) -> Arc<HashMap<PluginId, Arc<PluginRef>>> {
        let lock = self.internal.lock();
        lock.plugins_map.clone()
    }
    pub fn info(&self, message: &str) {
        let name = self
            .internal
            .lock()
            .plugins_map
            .get(&self.plugin_id)
            .expect("The plugin that called this should exist")
            .plugin_info
            .name
            .clone();
        println!("INFO [{}]: {}", name, message);
    }
    pub fn warn(&self, message: &str) {
        let name = self
            .internal
            .lock()
            .plugins_map
            .get(&self.plugin_id)
            .expect("The plugin that called this should exist")
            .plugin_info
            .name
            .clone();
        eprintln!("WARN [{}]: {}", name, message);
    }
    pub fn error(&self, message: &str) {
        let name = self
            .internal
            .lock()
            .plugins_map
            .get(&self.plugin_id)
            .expect("The plugin that called this should exist")
            .plugin_info
            .name
            .clone();
        eprintln!("ERROR [{}]: {}", name, message);
    }
    pub fn fatal(&self, message: &str) {
        let name = self
            .internal
            .lock()
            .plugins_map
            .get(&self.plugin_id)
            .expect("The plugin that called this should exist")
            .plugin_info
            .name
            .clone();
        eprintln!("FATAL [{}]: {}", name, message);
    }
}

impl<T: Event> EventSender<T> {
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
            hook.call_once((event,));
            // Box::leak(hook);
        }
    }
}
