use fruity_core::inject::Const;
use fruity_ecs::system::system_service::SystemService;

pub fn pause_at_startup(system_service: Const<SystemService>) {
    system_service.set_paused(true);
}
