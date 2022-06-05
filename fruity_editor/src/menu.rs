use crate::use_global;
use crate::MutationService;
use crate::SceneState;
use crate::WorldState;

pub fn open() {
    let scene_state = use_global::<SceneState>();
    scene_state.open();
}

pub fn is_save_enabled() -> bool {
    let scene_state = use_global::<SceneState>();
    scene_state.can_save()
}

pub fn save() {
    let scene_state = use_global::<SceneState>();
    scene_state.save();
}

pub fn save_as() {
    let scene_state = use_global::<SceneState>();
    scene_state.save();
}

pub fn is_undo_enabled() -> bool {
    let world_state = use_global::<WorldState>();
    let mutation_service = world_state.resource_container.require::<MutationService>();
    let mutation_service = mutation_service.read();

    mutation_service.can_undo()
}

pub fn undo() {
    let world_state = use_global::<WorldState>();
    let mutation_service = world_state.resource_container.require::<MutationService>();
    let mut mutation_service = mutation_service.write();

    mutation_service.undo();
}

pub fn is_redo_enabled() -> bool {
    let world_state = use_global::<WorldState>();
    let mutation_service = world_state.resource_container.require::<MutationService>();
    let mutation_service = mutation_service.read();

    mutation_service.can_redo()
}

pub fn redo() {
    let world_state = use_global::<WorldState>();
    let mutation_service = world_state.resource_container.require::<MutationService>();
    let mut mutation_service = mutation_service.write();

    mutation_service.redo();
}
