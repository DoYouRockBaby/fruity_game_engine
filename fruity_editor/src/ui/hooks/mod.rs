use crate::ui::context::UIContext;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReadGuard;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::resource::resource_reference::ResourceWriteGuard;
use fruity_core::RwLock;
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::Arc;

pub fn use_service<T: Resource + ?Sized>(ctx: &UIContext) -> ResourceReference<T> {
    let ctx_reader = ctx.read();
    ctx_reader.resource_container.require::<T>()
}

pub fn use_read_service<T: Resource + ?Sized>(ctx: &UIContext) -> ResourceReadGuard<T> {
    let service = use_service::<T>(ctx);
    service.read()
}

pub fn use_write_service<T: Resource + ?Sized>(ctx: &UIContext) -> ResourceWriteGuard<T> {
    let service = use_service::<T>(ctx);
    service.write()
}

pub fn use_effect<T: Hash + Eq + Clone + Send + Sync + 'static>(
    ctx: &mut UIContext,
    callback: impl FnOnce() -> Box<dyn FnOnce() + Send + Sync>,
    params: T,
) {
    let (stored_params, set_stored_params) = use_state::<Option<T>>(ctx, None);
    let (dispose_callback, set_dispose_callback) = use_state::<
        Arc<RwLock<Option<Box<dyn FnOnce() + Send + Sync>>>>,
    >(ctx, Arc::new(RwLock::new(None)));

    if let Some(stored_params) = stored_params {
        if stored_params != params {
            if let Some(dispose_callback) = dispose_callback.write().take() {
                dispose_callback();
                set_dispose_callback(Arc::new(RwLock::new(None)));
            }

            let dispose_callback = callback();
            set_dispose_callback(Arc::new(RwLock::new(Some(dispose_callback.into()))));
            set_stored_params(Some(params));
        }
    } else {
        let dispose_callback = callback();
        set_dispose_callback(Arc::new(RwLock::new(Some(dispose_callback.into()))));
    }
}

pub fn use_state<T: Clone + Send + Sync + 'static>(
    ctx: &mut UIContext,
    default: T,
) -> (T, impl Fn(T) -> T) {
    // Increment the local storage index, so the next use_state will use the next value in the storage
    let local_storage_current = ctx.local_storage_current;
    ctx.local_storage_current += 1;

    // Get the state index
    let mut ctx_writer = ctx.write();

    // Get or create the local storage associated with the current context
    let local_storage = ctx_writer
        .local_storages
        .entry(ctx.local_index.clone())
        .or_insert(Default::default());

    // Get the state value and modifier
    let value_shared_ref = if let Some(value) = local_storage.get(local_storage_current) {
        if let Some(value_shared_ref) = value.downcast_ref::<Arc<RwLock<T>>>() {
            value_shared_ref.clone()
        } else {
            let new_value = Arc::new(RwLock::new(default));
            let _ = std::mem::replace(
                &mut local_storage[local_storage_current],
                Box::new(new_value.clone()),
            );

            new_value
        }
    } else {
        let new_value = Arc::new(RwLock::new(default));
        local_storage.push(Box::new(new_value.clone()));

        new_value
    };

    let value = value_shared_ref.read();
    let value_shared_ref = value_shared_ref.clone();
    let modifier = move |new_value: T| {
        let mut writer = value_shared_ref.write();
        std::mem::replace(writer.deref_mut(), new_value)
    };

    (value.clone(), modifier)
}

pub fn use_memo<T, D>(
    ctx: &mut UIContext,
    get_value: impl FnOnce(&UIContext) -> T,
    dependencies: D,
) -> T
where
    T: Clone + Send + Sync + 'static,
    D: Eq + Clone + Send + Sync + 'static,
{
    let (value, set_value) = use_state::<Option<T>>(ctx, None);
    let (previous_dependencies, set_previous_dependencies) = use_state::<Option<D>>(ctx, None);

    if let Some(value) = value {
        if let Some(previous_dependencies) = previous_dependencies {
            if dependencies != previous_dependencies {
                let new_value = get_value(ctx);
                set_value(Some(new_value.clone()));
                set_previous_dependencies(Some(dependencies));

                new_value
            } else {
                value
            }
        } else {
            let new_value = get_value(ctx);
            set_value(Some(new_value.clone()));
            set_previous_dependencies(Some(dependencies));

            new_value
        }
    } else {
        let new_value = get_value(ctx);
        set_value(Some(new_value.clone()));
        set_previous_dependencies(Some(dependencies));

        new_value
    }
}
