pub trait Mutation: Send + Sync + 'static {
    fn apply(&self);
    fn undo(&self);
}
