use abi_stable::{
    StableAbi, declare_root_module_statics,
    external_types::RMutex,
    library::RootModule,
    package_version_strings,
    std_types::{RArc, RString},
};

use crate::internal_pluggie_context::InternalPluggieCtx;

#[derive(StableAbi, Clone)]
#[repr(C)]
pub struct PluginInfo {
    pub name: RString,
    pub version: RString,
    pub author: RString,
    pub pluggie_version: u32,
}

#[derive(StableAbi, Clone)]
#[repr(C)]
pub struct PluginRef {
    pub init: extern "C" fn(ctx: RArc<RMutex<InternalPluggieCtx>>),
    pub plugin_info: PluginInfo,
}

#[macro_export]
macro_rules! describe_plugin {
    ($init:ident, $info: expr) => {
        pub extern "C" fn __pluggie_init(
            ctx: abi_stable::std_types::RArc<
                abi_stable::external_types::RMutex<
                    pluggie::internal_pluggie_context::InternalPluggieCtx,
                >,
            >,
        ) {
            // this is here so that the compiler doesn't complain about unused imports for PluginInfo, same with the pub extern "C" in __pluggie_init
            let _ = $info;
            $init(PluggieCtx::new(ctx));
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
