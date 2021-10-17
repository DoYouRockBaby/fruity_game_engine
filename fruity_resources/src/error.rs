use crate::ResourceIdentifier;

pub enum ResourcesManagerError {
    ResourceAlreadyExists(ResourceIdentifier),
    ResourceNotFound(ResourceIdentifier),
}
