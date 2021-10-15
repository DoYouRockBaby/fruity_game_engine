use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

pub trait JsValue: Any + Debug {
    fn as_v8<'a>(&mut self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value>;
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}
