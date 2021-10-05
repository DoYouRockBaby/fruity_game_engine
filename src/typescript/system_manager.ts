export interface SystemManager {
    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    ///
    add_system(system: () => void): void;
}