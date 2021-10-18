use crate::resource::resources_manager::ResourceIdentifier;

/// Error that can occure whilte trying to add a resource in the resources manager
#[derive(Debug, Clone)]
pub enum AddResourceError {
    /// The resource already exists
    ResourceAlreadyExists(ResourceIdentifier),
}

/// Error that can occure whilte trying to load a resource with the resources manager
#[derive(Debug, Clone)]
pub enum LoadResourceError {
    /// The resource already exists
    ResourceAlreadyExists(ResourceIdentifier),
    /// The resource type is not known, a resource loader should be added to handle this type of file
    ResourceTypeNotKnown(String),
}

/// Error that can occure whilte trying to remove a resource from the resources manager
#[derive(Debug, Clone)]
pub enum RemoveResourceError {
    /// The resource not exists
    ResourceNotFound(ResourceIdentifier),
}

/// Error that can occure whilte trying to add a resource loader for a given type in the resources manager
#[derive(Debug, Clone)]
pub enum AddResourceLoaderError {
    /// The resource type already exists
    ResourceTypeAlreadyExists(String),
}

impl From<AddResourceError> for LoadResourceError {
    fn from(error: AddResourceError) -> Self {
        match error {
            AddResourceError::ResourceAlreadyExists(identifier) => {
                LoadResourceError::ResourceAlreadyExists(identifier)
            }
        }
    }
}
