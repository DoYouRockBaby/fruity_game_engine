use crate::use_global;
use crate::MutationService;
use crate::WorldState;

pub trait Mutation: Send + Sync + 'static {
    fn apply(&self);
    fn undo(&self);
}
