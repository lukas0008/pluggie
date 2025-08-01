use std::{convert::identity, sync::Arc};

use abi_stable::{external_types::RMutex, std_types::RArc};
use pluggie::{
    AllLoadedEvent,
    internal_pluggie_context::InternalPluggieCtx,
    pluggie_context::PluggieCtx,
    plugin::{PluginId, PluginRef},
};

fn main() {
    #[cfg(target_os = "macos")]
    let os_ext = "dylib";
    #[cfg(target_os = "linux")]
    let os_ext = "so";

    let plugin_paths = std::fs::read_dir("./plugins").unwrap();
    let mut paths = Vec::new();
    for plugin in plugin_paths {
        match plugin {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == os_ext {
                    paths.push(path);
                }
            }
            Err(e) => eprintln!("Error reading plugin directory: {}", e),
        }
    }

    println!(
        "INFO [pluggie]: Loading the following libraries: {}",
        paths
            .iter()
            .filter_map(|v| v.file_name().map(|v| v.to_str()))
            .filter_map(identity)
            .collect::<Vec<_>>()
            .join(", ")
    );

    let plugins = paths
        .into_iter()
        .map(|path| {
            let lib = unsafe { libloading::Library::new(&path) }
                .expect(&format!("Failed to load plugin: {}", path.display()));
            (lib, path)
        })
        .collect::<Vec<_>>();
    // let lib = Plugin_Ref::load_from_file(&path);

    // let plugins = plugins.into_iter().map(Arc::new).collect::<Vec<_>>();

    // extern "C" fn(RArc<RMutex<InternalPluggieCtx>>)
    let mut plugin_refs = Vec::new();
    for (plugin, _path) in plugins.iter() {
        // let init: libloading::Symbol<extern "C" fn(RArc<RMutex<InternalPluggieCtx>>)> =
        //     unsafe { plugin.get(b"pluggie_init") }.unwrap();
        // init(ctx.clone());
        // println!("{}", path.file_name().unwrap().to_str().unwrap());
        let init: libloading::Symbol<extern "C" fn() -> PluginRef> =
            unsafe { plugin.get(b"pluggie_def") }.unwrap();
        let plugin = init();
        // (plugin.init)(local_ctx.clone());
        println!("INFO [pluggie]: {} loaded", plugin.plugin_info.name);
        if plugin.plugin_info.pluggie_version != pluggie::VERSION {
            panic!(
                "Plugin {} has incompatible version",
                plugin.plugin_info.name
            );
        }
        plugin_refs.push(Arc::new(plugin));
    }
    let ctx = RArc::new(RMutex::new(InternalPluggieCtx::new(plugin_refs.clone())));
    let local_ctx = PluggieCtx::new(ctx.clone(), PluginId::PLUGGIE_ID);
    let loaded_event = local_ctx.register_event::<AllLoadedEvent>();

    {
        let plugin_map = local_ctx.get_plugin_map();
        for (plugin_id, plugin) in plugin_map.iter().filter(|(_, plugin)| plugin.load_early) {
            (plugin.init)(local_ctx.clone_with_plugin_id(*plugin_id));
        }
        for (plugin_id, plugin) in plugin_map.iter().filter(|(_, plugin)| !plugin.load_early) {
            (plugin.init)(local_ctx.clone_with_plugin_id(*plugin_id));
        }
    }

    loaded_event.call(&AllLoadedEvent {
        plugins: plugin_refs,
    });
}
