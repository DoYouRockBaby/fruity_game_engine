/// Error that can occure whilte trying to add a resource in the resources manager
#[derive(Debug, Clone)]
pub enum AddResourceError {
    /// The resource already exists
    ResourceAlreadyExists(String),
}

impl ToString for AddResourceError {
    fn to_string(&self) -> String {
        match self {
            AddResourceError::ResourceAlreadyExists(name) => {
                format!("Resource named \"{}\" already exists", &name)
            }
        }
    }
}

/// Error that can occure whilte trying to load a resource with the resources manager
#[derive(Debug, Clone)]
pub enum LoadResourceError {
    /// The resource already exists
    ResourceAlreadyExists(String),
    /// The resource type is not known, a resource loader should be added to handle this type of file
    ResourceTypeNotKnown(String),
}

impl ToString for LoadResourceError {
    fn to_string(&self) -> String {
        match self {
            LoadResourceError::ResourceAlreadyExists(name) => {
                format!("Resource named \"{}\" already exists", &name)
            }
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

impl From<AddResourceError> for LoadResourceError {
    fn from(error: AddResourceError) -> Self {
        match error {
            AddResourceError::ResourceAlreadyExists(identifier) => {
                LoadResourceError::ResourceAlreadyExists(identifier)
            }
        }
    }
}
