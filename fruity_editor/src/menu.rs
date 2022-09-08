use crate::ui::context::UIContext;
use crate::ui::hooks::use_read_service;
use crate::ui::hooks::use_write_service;
use crate::MutationService;
use crate::SceneState;

pub fn open(ctx: &UIContext) {
    let mut scene_state = use_write_service::<SceneState>(ctx);
    scene_state.open();
}

pub fn is_save_enabled(ctx: &UIContext) -> bool {
    let scene_state = use_read_service::<SceneState>(ctx);
    scene_state.can_save()
}

pub fn save(ctx: &UIContext) {
    let mut scene_state = use_write_service::<SceneState>(ctx);
    scene_state.save();
}

pub fn save_as(ctx: &UIContext) {
    let mut scene_state = use_write_service::<SceneState>(ctx);
    scene_state.save();
}

pub fn is_undo_enabled(ctx: &UIContext) -> bool {
    let mutation_service = use_read_service::<MutationService>(ctx);
    mutation_service.can_undo()
}

pub fn undo(ctx: &UIContext) {
    let mut mutation_service = use_write_service::<MutationService>(ctx);
    mutation_service.undo();
}

pub fn is_redo_enabled(ctx: &UIContext) -> bool {
    let mutation_service = use_read_service::<MutationService>(ctx);
    mutation_service.can_redo()
}

pub fn redo(ctx: &UIContext) {
    let mut mutation_service = use_write_service::<MutationService>(ctx);
    mutation_service.redo();
}
