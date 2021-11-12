use crate::JavascriptEngine;
use crate::ResourceManager;
use fruity_core::settings::Settings;
use std::io::Read;
use std::sync::Arc;

pub fn load_js_script(
    identifier: &str,
    _reader: &mut dyn Read,
    _params: Settings,
    resource_manager: Arc<ResourceManager>,
) {
    let javascript_engine = resource_manager.require::<JavascriptEngine>("javascript_engine");

    let javascript_engine = javascript_engine.read();
    javascript_engine.run_module(identifier);
}
