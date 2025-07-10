use std::sync::Arc;

use abi_stable::{external_types::RMutex, std_types::RArc};
use pluggie::{
    AllLoadedEvent, internal_pluggie_context::InternalPluggieCtx, pluggie_context::PluggieCtx,
};

fn main() {
    let plugin_paths = std::fs::read_dir("./plugins").unwrap();
    let mut plugins = Vec::new();

    for plugin in plugin_paths {
        match plugin {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    let lib =
                        unsafe { libloading::Library::new(&path) }.expect("Failed to load plugin");
                    // let lib = Plugin_Ref::load_from_file(&path);
                    plugins.push(lib);
                }
            }
            Err(e) => eprintln!("Error reading plugin directory: {}", e),
        }
    }

    let plugins = Arc::new(plugins);
    let ctx = RArc::new(RMutex::new(InternalPluggieCtx::new()));

    let local_ctx = PluggieCtx::new(ctx.clone());
    let loaded_event = local_ctx.register_event::<AllLoadedEvent>();

    for plugin in plugins.iter() {
        let init: libloading::Symbol<extern "C" fn(RArc<RMutex<InternalPluggieCtx>>)> =
            unsafe { plugin.get(b"pluggie_init") }.unwrap();
        init(ctx.clone());
    }

    loaded_event.call(&AllLoadedEvent);
}
