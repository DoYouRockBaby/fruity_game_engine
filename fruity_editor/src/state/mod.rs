use crate::components::panes::PanesMessage;
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

pub mod entity;
pub mod theme;
pub mod world;

#[derive(Clone)]
pub enum Message {
    Empty,
    Panes(PanesMessage),
    Callback(Arc<Mutex<dyn FnMut() + Send + Sync>>),
    StringChanged(Arc<Mutex<dyn FnMut(&str) + Send + Sync>>, String),
    BoolChanged(Arc<Mutex<dyn FnMut(bool) + Send + Sync>>, bool),
    IntegerChanged(Arc<Mutex<dyn FnMut(i64) + Send + Sync>>, i64),
    FloatChanged(Arc<Mutex<dyn FnMut(f64) + Send + Sync>>, f64),
    AnyChanged(
        Arc<Mutex<dyn FnMut(&dyn Any) + Send + Sync>>,
        Arc<dyn Any + Send + Sync>,
    ),
}

impl Debug for Message {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

pub fn handle_message(message: Message) {
    match message {
        Message::Callback(callback) => {
            let mut callback = callback.lock().unwrap();
            callback()
        }
        Message::StringChanged(callback, value) => {
            let mut callback = callback.lock().unwrap();
            callback(&value)
        }
        Message::BoolChanged(callback, value) => {
            let mut callback = callback.lock().unwrap();
            callback(value)
        }
        Message::IntegerChanged(callback, value) => {
            let mut callback = callback.lock().unwrap();
            callback(value)
        }
        Message::FloatChanged(callback, value) => {
            let mut callback = callback.lock().unwrap();
            callback(value)
        }
        Message::AnyChanged(callback, value) => {
            let mut callback = callback.lock().unwrap();
            callback(value.deref())
        }
        _ => (),
    }
}
