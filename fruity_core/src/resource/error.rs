/// Error that can occure whilte trying to load a resource with the resources manager
#[derive(Debug, Clone)]
pub enum LoadResourceError {
    /// The resource type is not known, a resource loader should be added to handle this type of file
    ResourceTypeNotKnown(String),
}

impl ToString for LoadResourceError {
    fn to_string(&self) -> String {
        match self {
            LoadResourceError::ResourceTypeNotKnown(name) => {
                format!(
                    "Resource type \"{}\" is not supported, maybe you forgot to include a module",
                    &name
                )
            }
        }
    }
}

/// Error that can occure whilte trying to remove a resource from the resources manager
#[derive(Debug, Clone)]
pub enum RemoveResourceError {
    /// The resource not exists
    ResourceNotFound(String),
}

impl ToString for RemoveResourceError {
    fn to_string(&self) -> String {
        match self {
            RemoveResourceError::ResourceNotFound(name) => {
                format!("Resource named \"{}\" not exists", &name)
            }
        }
    }
}
