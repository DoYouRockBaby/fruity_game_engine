use std::fmt::Debug;
use std::sync::mpsc;
use std::thread;

type Callback<T, R> = dyn FnOnce(&mut T) -> R + Send + Sync + 'static;

struct CallInstruction<T: Debug + 'static, R = ()> {
    callback: Box<Callback<T, R>>,
    notify_done_sender: mpsc::Sender<R>,
}

/// A tool to simplify the exposition of a single threaded module into a Send Sync wrapper
#[derive(Debug)]
pub struct SingleThreadWrapper<T: Debug + 'static, R: Sync + Send + 'static = ()> {
    channel_sender: mpsc::SyncSender<CallInstruction<T, R>>,
}

impl<T: Debug + 'static, R: Sync + Send + 'static> SingleThreadWrapper<T, R> {
    /// Initialize the thread
    /// Mostly construct the inner instance and run it's thread
    ///
    /// # Arguments
    /// * `constructor` - The function that will construct the inner instance
    ///
    pub fn start<F>(constructor: F) -> SingleThreadWrapper<T, R>
    where
        F: FnOnce() -> T + Send + Sync + 'static,
    {
        // TODO: think about a good number for sync channel
        let (sender, receiver) = mpsc::sync_channel::<CallInstruction<T, R>>(10);
        let (loading_sender, loading_receiver) = mpsc::channel::<()>();

        // Create a thread that will be dedicated to the inner instance
        // An event channel will be used to send instruction to the instance
        thread::spawn(move || {
            let mut inner = constructor();
            loading_sender.send(()).unwrap();

            for received in receiver {
                let result = (received.callback)(&mut inner);
                (received.notify_done_sender).send(result).unwrap();
            }
        });

        loading_receiver.recv().unwrap();

        SingleThreadWrapper {
            channel_sender: sender,
        }
    }

    /// Call a intern method
    ///
    /// # Arguments
    /// * `callback` - The function that will be called in the inner instance
    ///
    pub fn call<F>(&self, callback: F) -> R
    where
        F: Fn(&mut T) -> R + Send + Sync + 'static,
    {
        let (notify_done_sender, notify_done_receiver) = mpsc::channel::<R>();

        self.channel_sender
            .send(CallInstruction {
                callback: Box::new(callback),
                notify_done_sender,
            })
            .unwrap();

        notify_done_receiver.recv().unwrap()
    }
}
