use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::system::system_service::SystemService;
use std::sync::Arc;

pub fn pause_at_startup(resource_container: Arc<ResourceContainer>) {
    let system_service = resource_container.require::<SystemService>("system_service");
    let system_service = system_service.read();

    system_service.set_paused(true);
}
