use abi_stable::{StableAbi, std_types::RString};

use crate::pluggie_context::PluggieCtx;

#[derive(StableAbi, Clone)]
#[repr(C)]
pub struct PluginInfo {
    pub name: RString,
    pub version: RString,
    pub author: RString,
    pub pluggie_version: u32,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, StableAbi)]
pub struct PluginId(pub(crate) u32);

impl PluginId {
    pub const PLUGGIE_ID: PluginId = PluginId(0);
}

#[derive(Clone)]
#[repr(C)]
pub struct PluginRef {
    pub init: extern "C" fn(ctx: PluggieCtx),
    pub plugin_info: PluginInfo,
}

#[macro_export]
macro_rules! describe_plugin {
    ($init:ident, $info: expr) => {
        pub extern "C" fn __pluggie_init(
            // ctx: pluggie::reexports::abi_stable::std_types::RArc<
            //     pluggie::reexports::abi_stable::external_types::RMutex<
            //         pluggie::internal_pluggie_context::InternalPluggieCtx,
            //     >,
            // >,
            ctx: pluggie::pluggie_context::PluggieCtx,
        ) {
            // this is here so that the compiler doesn't complain about unused imports for PluginInfo, same with the pub extern "C" in __pluggie_init
            let _ = $info;
            $init(ctx);
        }

        #[unsafe(no_mangle)]
        #[cfg(feature = "init")]
        pub extern "C" fn pluggie_def() -> pluggie::plugin::PluginRef {
            pluggie::plugin::PluginRef {
                init: __pluggie_init,
                plugin_info: $info,
            }
        }
    };
}
