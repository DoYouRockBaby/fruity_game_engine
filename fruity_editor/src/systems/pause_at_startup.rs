use fruity_core::service::service_manager::ServiceManager;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;
use std::sync::RwLock;

pub fn pause_at_startup(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let system_manager = service_manager.read::<SystemManager>();
    system_manager.set_paused(true);
}
