use fruity_ecs::world::world::World;
use fruity_ecs::system::system_manager::SystemManager;
use wasmer::{Store, Module, Instance};
use wasmer_wasi::WasiState;

pub fn execute_script(world: &mut World, module_wat: impl AsRef<[u8]>) -> Result<(), ()> {
    let store = Store::default();
    
    let module = match Module::new(&store, &module_wat) {
        Ok(module) => module,
        Err(err) => panic!("{}", err.to_string()),
    };

    // Create the `WasiEnv`.
    let mut wasi_env = match WasiState::new("script").finalize() {
        Ok(wasi_env) => wasi_env,
        Err(err) => panic!("{}", err.to_string()),
    };

    // Generate an `ImportObject`.
    let import_object = match wasi_env.import_object(&module) {
        Ok(import_object) => import_object,
        Err(err) => panic!("{}", err.to_string()),
    };
    
    // Let's instantiate the module with the imports.
    let instance = match Instance::new(&module, &import_object) {
        Ok(instance) => instance,
        Err(err) => panic!("{}", err.to_string()),
    };


    // Let's call the `initialize` function
    let initialize = match instance.exports.get_function("initialize") {
        Ok(initialize) => initialize,
        Err(err) => panic!("{}", err.to_string()),
    };

    match initialize.call(&[/*&mut world.system_manager*/]) {
        Ok(_) => {},
        Err(err) => panic!("{}", err.to_string()),
    };

    Ok(())
}