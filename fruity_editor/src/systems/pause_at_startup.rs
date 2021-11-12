use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;

pub fn pause_at_startup(resource_manager: Arc<ResourceManager>) {
    let system_manager = resource_manager.require::<SystemManager>("system_manager");
    let system_manager = system_manager.read();

    system_manager.set_paused(true);
}
