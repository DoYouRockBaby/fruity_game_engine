use crate::JavascriptService;
use crate::ResourceContainer;
use fruity_core::settings::Settings;
use std::io::Read;
use std::sync::Arc;

pub fn load_js_script(
    identifier: &str,
    _reader: &mut dyn Read,
    _params: Settings,
    resource_container: Arc<ResourceContainer>,
) {
    let javascript_service = resource_container.require::<JavascriptService>();

    let javascript_service = javascript_service.read();
    javascript_service.run_module(identifier);
}
