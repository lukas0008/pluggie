use abi_stable::{
    StableAbi, declare_root_module_statics,
    external_types::RMutex,
    library::RootModule,
    package_version_strings,
    std_types::{RArc, RString},
};

use crate::internal_pluggie_context::InternalPluggieCtx;

#[derive(StableAbi)]
#[repr(C)]
pub struct PluginInfo {
    pub name: RString,
    pub version: RString,
    pub author: RString,
    pub pluggie_version: u32,
}

#[derive(StableAbi)]
#[sabi(kind(Prefix))]
#[repr(C)]
pub struct Plugin {
    pub init: extern "C" fn(ctx: RArc<RMutex<InternalPluggieCtx>>),
    pub plugin_info: extern "C" fn() -> PluginInfo,
}

impl RootModule for Plugin_Ref {
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: abi_stable::sabi_types::VersionStrings = package_version_strings!();

    declare_root_module_statics! {Plugin_Ref}
}
