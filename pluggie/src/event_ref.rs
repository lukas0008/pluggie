use std::{ops::Deref, sync::Arc};

use abi_stable::{StableAbi, std_types::RArc};

use crate::{event::Event, event_hooks::EventHooks};

#[derive(StableAbi, Clone)]
#[repr(C)]
pub struct EventRef<'a, T: Event + Clone> {
    pub(crate) event: &'a T,
    pub(crate) hooks: RArc<EventHooks<T>>,
}

impl<'a, T: Event + Clone> EventRef<'a, T> {
    pub fn post_event_hook(&self, hook: impl Fn(&T) + 'static) {
        self.hooks.add_hook(Box::new(hook));
    }
}

impl<'a, T: Event + Clone> Deref for EventRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.event
    }
}
