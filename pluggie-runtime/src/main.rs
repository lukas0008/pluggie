use abi_stable::{external_types::RMutex, library::RootModule, std_types::RArc};
use pluggie::{
    AllLoadedEvent, internal_pluggie_context::InternalPluggieCtx, pluggie_context::PluggieCtx,
    plugin::Plugin_Ref,
};

fn main() {
    let plugin_paths = std::fs::read_dir("./plugins").unwrap();
    let mut plugins = Vec::new();

    for plugin in plugin_paths {
        match plugin {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    
                    let lib = Plugin_Ref::load_from_file(&path);
                    plugins.push(lib.unwrap());
                }
            }
            Err(e) => eprintln!("Error reading plugin directory: {}", e),
        }
    }
    let ctx = RArc::new(RMutex::new(InternalPluggieCtx::new()));

    let local_ctx = PluggieCtx::new(ctx.clone());
    let loaded_event = local_ctx.register_event::<AllLoadedEvent>();

    for plugin in plugins {
        let init = plugin.init().unwrap();
        init(ctx.clone());
    }

    loaded_event.call(&AllLoadedEvent);
}
