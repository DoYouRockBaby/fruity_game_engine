use crate::runtime::JsRuntimeHandles;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

pub trait JsValue: Any + Debug {
    fn as_v8(&mut self, handles: Arc<Mutex<JsRuntimeHandles>>) -> v8::Local<v8::Value>;
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}
