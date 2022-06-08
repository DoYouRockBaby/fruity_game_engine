use crate::Rapier2dService;
use fruity_core::inject::Mut;

pub fn update_physics(mut rapier_2d_service: Mut<Rapier2dService>) {
    rapier_2d_service.update();
}
