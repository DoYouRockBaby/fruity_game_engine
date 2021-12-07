use crate::introspect::FieldInfo;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::resource::error::LoadResourceError;
use crate::resource::error::RemoveResourceError;
use crate::resource::resource::Resource;
use crate::resource::resource_reference::AnyResourceReference;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::serialized_resource::SerializedResource;
use crate::serialize::serialized::Serialized;
use crate::settings::Settings;
use crate::utils::introspect::cast_introspect_ref;
use crate::utils::introspect::ArgumentCaster;
use fruity_any::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

/// A a function that is used to load a resource
pub type ResourceLoader = fn(&str, &mut dyn Read, Settings, Arc<ResourceContainer>);

/// The resource manager
#[derive(FruityAny, Clone)]
pub struct ResourceContainer {
    pub(crate) inner: Arc<RwLock<InnerResourceContainer>>,
}

impl Debug for ResourceContainer {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

pub(crate) struct InnerResourceContainer {
    resources: HashMap<String, Arc<dyn Resource>>,
    identifier_by_type: HashMap<TypeId, String>,
    resource_loaders: HashMap<String, ResourceLoader>,
}

impl ResourceContainer {
    /// Returns a ResourceContainer
    pub fn new() -> ResourceContainer {
        ResourceContainer {
            inner: Arc::new(RwLock::new(InnerResourceContainer {
                resources: HashMap::new(),
                identifier_by_type: HashMap::new(),
                resource_loaders: HashMap::new(),
            })),
        }
    }

    /// Get a required resource by it's identifier
    /// Panic if the resource is not known
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn require<T: Resource + ?Sized>(&self) -> ResourceReference<T> {
        let inner = self.inner.read().unwrap();

        match inner.identifier_by_type.get(&TypeId::of::<T>()) {
            Some(resource_name) => match inner.resources.get(resource_name) {
                Some(resource) => {
                    match resource.clone().as_any_arc().downcast::<RwLock<Box<T>>>() {
                        Ok(resource) => {
                            ResourceReference::new(resource_name, resource, Arc::new(self.clone()))
                        }
                        Err(_) => {
                            panic!("Failed to get a required resource")
                        }
                    }
                }
                None => {
                    panic!("Failed to get a required resource")
                }
            },
            None => {
                panic!("Failed to get a required resource")
            }
        }
    }

    /// Get a resource by it's identifier
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn get<T: Resource + ?Sized>(&self, identifier: &str) -> Option<ResourceReference<T>> {
        let inner = self.inner.read().unwrap();

        match inner
            .resources
            .get(identifier)
            .map(|resource| resource.clone())
        {
            Some(resource) => match resource.as_any_arc().downcast::<RwLock<Box<T>>>() {
                Ok(resource) => Some(ResourceReference::new(
                    identifier,
                    resource,
                    Arc::new(self.clone()),
                )),
                Err(_) => None,
            },
            None => None,
        }
    }

    /// Get a resource by it's identifier without casting it
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn get_untyped(&self, identifier: &str) -> Option<AnyResourceReference> {
        let inner = self.inner.read().unwrap();

        inner.resources.get(identifier).map(|resource| {
            AnyResourceReference::new(identifier, resource.clone(), Arc::new(self.clone()))
        })
    }

    /// Check if a resource identifier has already been registered
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn contains(&self, identifier: &str) -> bool {
        let inner = self.inner.read().unwrap();
        inner.resources.contains_key(identifier)
    }

    /// Add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource` - The resource object
    ///
    pub fn add<T: Resource + ?Sized>(&self, identifier: &str, resource: Box<T>) {
        let mut inner = self.inner.write().unwrap();

        let shared = Arc::new(RwLock::new(resource));
        inner
            .resources
            .insert(identifier.to_string(), shared.clone());
        inner
            .identifier_by_type
            .insert(TypeId::of::<T>(), identifier.to_string());
    }

    /// Remove a resource of the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn remove(&self, identifier: &str) -> Result<(), RemoveResourceError> {
        let mut inner = self.inner.write().unwrap();

        if inner.resources.contains_key(identifier) {
            inner.resources.remove(identifier);

            Ok(())
        } else {
            Err(RemoveResourceError::ResourceNotFound(
                identifier.to_string(),
            ))
        }
    }

    /// Add a resource loader that will be used to load resources
    ///
    /// # Arguments
    /// * `resource_type` - The resource loader type
    /// * `loader` - The resource loader
    ///
    pub fn add_resource_loader(&self, resource_type: &str, loader: ResourceLoader) {
        let mut inner = self.inner.write().unwrap();
        inner
            .resource_loaders
            .insert(resource_type.to_string(), loader);
    }

    /// Load an any resource file
    ///
    /// # Arguments
    /// * `path` - The path of the file
    /// * `resource_type` - The resource type
    ///
    pub fn load_resource_file(
        &self,
        path: &str,
        resource_type: &str,
    ) -> Result<(), LoadResourceError> {
        let mut file = File::open(path).unwrap();
        Self::load_resource(self, path, resource_type, &mut file, Settings::new())?;

        Ok(())
    }

    /// Load and add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource_type` - The resource type
    /// * `read` - The reader, generaly a file reader
    ///
    pub fn load_resource(
        &self,
        identifier: &str,
        resource_type: &str,
        reader: &mut dyn Read,
        settings: Settings,
    ) -> Result<(), LoadResourceError> {
        let resource_loader = {
            let inner_reader = self.inner.read().unwrap();

            if let Some(resource_loader) = inner_reader.resource_loaders.get(resource_type) {
                Ok(resource_loader.clone())
            } else {
                Err(LoadResourceError::ResourceTypeNotKnown(
                    resource_type.to_string(),
                ))
            }?
        };

        resource_loader(identifier, reader, settings, Arc::new(self.clone()));
        Ok(())
    }

    /// Load many resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resources_settings(&self, settings: Vec<Settings>) {
        settings.into_iter().for_each(|settings| {
            Self::load_resource_settings(self, settings);
        })
    }

    /// Load resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resource_settings(&self, settings: Settings) -> Option<()> {
        // Parse settings
        let fields = if let Settings::Object(fields) = settings {
            fields
        } else {
            return None;
        };

        // Get the resource name
        let name = {
            if let Settings::String(name) = fields.get("name")? {
                name.clone()
            } else {
                return None;
            }
        };

        // Get the resource path
        let path = {
            if let Settings::String(path) = fields.get("path")? {
                path.clone()
            } else {
                return None;
            }
        };

        // Deduce informations about the resource from the path
        let resource_type = Path::new(&path).extension()?;
        let resource_type = resource_type.to_str()?;
        let mut resource_file = File::open(&path).ok()?;

        // Load the resource
        Self::load_resource(
            self,
            &name,
            resource_type,
            &mut resource_file,
            Settings::Object(fields.clone()),
        )
        .ok()?;

        Some(())
    }
}

impl IntrospectObject for ResourceContainer {
    fn get_class_name(&self) -> String {
        "ResourceContainer".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<ResourceContainer>(this);

                    let mut caster = ArgumentCaster::new("get", args);
                    let arg1 = caster.cast_next::<String>()?;

                    let result = this.get_untyped(&arg1);

                    Ok(result.map(|result| Serialized::NativeObject(Box::new(result))))
                })),
            },
            MethodInfo {
                name: "add".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<ResourceContainer>(this);

                    let mut caster = ArgumentCaster::new("add", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.next()?;

                    this.add::<SerializedResource>(&arg1, Box::new(SerializedResource::new(arg2)));

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "remove".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<ResourceContainer>(this);

                    let mut caster = ArgumentCaster::new("remove", args);
                    let arg1 = caster.cast_next::<String>()?;

                    this.remove(&arg1).unwrap();

                    Ok(None)
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
