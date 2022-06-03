use crate::mutations::mutation::Mutation;

impl<T1: Mutation, T2: Mutation> Mutation for (T1, T2) {
    fn apply(&self) {
        self.0.apply();
        self.1.apply();
    }

    fn undo(&self) {
        self.0.undo();
        self.1.undo();
    }
}
