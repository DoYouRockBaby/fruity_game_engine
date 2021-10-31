use crate::service::service::Service;
use fruity_any::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

type ServiceCallback<T> = dyn Fn(&mut T) + Send + Sync + 'static;

struct CallInstruction<T: Debug + 'static> {
    callback: Arc<ServiceCallback<T>>,
    notify_done_sender: mpsc::Sender<()>,
}

/// A structure to manage module loading, supports hot reload
#[derive(Debug)]
pub struct SingleThreadService<T: Debug + 'static> {
    channel_sender: mpsc::SyncSender<CallInstruction<T>>,
}

impl<T: Debug + 'static> SingleThreadService<T> {
    /// Initialize the service
    /// Mostly construct the inner service and run it's thread
    ///
    /// # Arguments
    /// * `constructor` - The function that will construct the inner service
    ///
    pub fn initialize<F>(&self, constructor: F) -> SingleThreadService<T>
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        // TODO: think about a good number for sync channel
        let (sender, receiver) = mpsc::sync_channel::<CallInstruction<T>>(10);
        let (loading_sender, loading_receiver) = mpsc::channel::<()>();

        // Create a thread that will be dedicated to the inner service
        // An event channel will be used to send instruction to the service
        thread::spawn(move || {
            let mut inner_service = constructor();
            loading_sender.send(()).unwrap();

            for received in receiver {
                (received.callback)(&mut inner_service);
                (received.notify_done_sender).send(()).unwrap();
            }
        });

        loading_receiver.recv().unwrap();

        SingleThreadService {
            channel_sender: sender,
        }
    }

    /// Call a service method
    ///
    /// # Arguments
    /// * `callback` - The function that will be called in the inner service
    ///
    pub fn call<F>(&self, callback: F)
    where
        F: Fn(&mut T) + Send + Sync + 'static,
    {
        let (notify_done_sender, notify_done_receiver) = mpsc::channel::<()>();

        self.channel_sender
            .send(CallInstruction {
                callback: Arc::new(callback),
                notify_done_sender,
            })
            .unwrap();

        notify_done_receiver.recv().unwrap()
    }
}

// TODO: Improve the macro to handle the generics
impl<T: Debug + 'static> FruityAny for SingleThreadService<T> {
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
        self
    }
}

impl<T: Debug + 'static> Drop for SingleThreadService<T> {
    fn drop(&mut self) {}
}

impl<T: Debug + 'static> IntrospectObject for SingleThreadService<T> {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: Debug + 'static> Service for SingleThreadService<T> {}
