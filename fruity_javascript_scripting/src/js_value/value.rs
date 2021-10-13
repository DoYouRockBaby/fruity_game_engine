use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

pub trait JsValue: Any + Debug {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>);
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}
