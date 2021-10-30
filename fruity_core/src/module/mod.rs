use crate::World;
use hot_reload_lib::HotReloadLib;
use libloading::Library;
use libloading::Symbol;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use topological_sort::TopologicalSort;

/// An error that can occure during modules loading
#[derive(Debug, Clone)]
pub enum LoadModuleError {
    /// A file couln't not be read
    FileReadFailed(String),
    /// A module has an incorrect format
    IncorrectModule(String),
}

#[derive(Clone)]
struct Module<'a> {
    name: String,
    dependencies: Vec<String>,
    initialize: Symbol<'a, for<'r> unsafe extern "C" fn(&'r World)>,
}

pub struct Modules {
    modules: HashMap<String, HotReloadLib>,
}

impl<'a> PartialEq for Module<'a> {
    fn eq(&self, other: &Module) -> bool {
        self.name.eq(&other.name)
    }
}

impl<'a> Eq for Module<'a> {}

impl<'a> Hash for Module<'a> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.name.hash(hasher)
    }
}

/// Load dynamic modules contained in a folder
///
/// # Arguments
/// * `world` - The world instance
/// * `folder` - The folder where the modules are stored
///
pub fn load_modules(_world: &World, folder: &str) -> Result<(), LoadModuleError> {
    let dynlibs = load_dynlib_in_directory(folder)?;
    let modules = list_modules_in_directory(&dynlibs)?;
    let modules = order_modules_by_dependencies(modules);

    for module in modules {
        println!("{}", module.name);
    }

    Ok(())
}

fn order_modules_by_dependencies<'a>(modules: Vec<Module<'a>>) -> Vec<Module<'a>> {
    let mut ts = TopologicalSort::<Module<'a>>::new();
    for module in &modules {
        let dependencies = module
            .dependencies
            .iter()
            .filter_map(|dependency| modules.iter().find(|module| module.name.eq(dependency)))
            .collect::<Vec<_>>();

        for dependency in dependencies {
            ts.add_dependency(dependency.clone(), module.clone())
        }
    }

    ts.collect::<Vec<Module<'a>>>()
}

fn list_modules_in_directory<'a>(
    dynlibs: &'a [(String, Library)],
) -> Result<Vec<Module<'a>>, LoadModuleError> {
    let modules = dynlibs
    .iter()
        .filter_map(|(path, dynlib)| {
            let identifier_func: libloading::Symbol<unsafe extern "C" fn() -> &'static str> = unsafe{dynlib
                .get(b"identifier")
                .map_err(|_| LoadModuleError::IncorrectModule(path.clone()))
                .ok()?};

            let dependencies_func: libloading::Symbol<
                unsafe extern "C" fn() -> &'static [&'static str],
            > = unsafe {
                dynlib.get(b"dependencies")
                    .map_err(|_| LoadModuleError::IncorrectModule(path.clone()))
                    .ok()?
            };

            let initialize_func: libloading::Symbol<unsafe extern "C" fn(&World)> = unsafe {
                dynlib.get(b"initialize")
                    .map_err(|_| LoadModuleError::IncorrectModule(path.clone()))
                    .ok()?
            };

            Some(Module {
                name: unsafe { identifier_func().to_string() },
                dependencies: unsafe{ dependencies_func()}
                    .iter()
                    .map(|dependency| dependency.to_string())
                    .collect(),
                initialize: initialize_func,
            })
        })
        .collect::<Vec<_>>();

    Ok(modules)
}

fn load_dynlib_in_directory<'a>(folder: &str) -> Result<Vec<(String, Library)>, LoadModuleError> {
    let paths = fs::read_dir(folder).unwrap();
    let dynlibs = paths
        .filter(|path| {
            if let Some(extension) = path.as_ref().unwrap().path().extension() {
                // TODO: Constrain with the current OS
                extension == "dll" || extension == "so" || extension == "dylib"
            } else {
                false
            }
        })
        .filter_map(|path| {
            let path = path.unwrap().path().display().to_string();

            let lib = unsafe {
                libloading::Library::new(path.clone())
                    .map_err(|_| LoadModuleError::FileReadFailed(path.clone()))
                    .ok()?
            };

            Some((path.clone(), lib))
        })
        .collect::<Vec<_>>();

    Ok(dynlibs)
}
