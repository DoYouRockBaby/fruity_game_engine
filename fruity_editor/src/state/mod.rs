use std::fmt::Debug;
use std::sync::Arc;

pub mod entity;
pub mod theme;
pub mod world;

#[derive(Clone)]
pub enum Message {
    Empty,
    Callback(Arc<dyn Fn() + Send + Sync>),
    StringChanged(Arc<dyn Fn(&str) + Send + Sync>, String),
    BoolChanged(Arc<dyn Fn(bool) + Send + Sync>, bool),
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
        Message::Callback(callback) => callback(),
        Message::StringChanged(callback, value) => callback(&value),
        Message::BoolChanged(callback, value) => callback(value),
        _ => (),
    }
}
