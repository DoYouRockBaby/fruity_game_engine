use rusty_v8 as v8;
use std::fmt::Debug;

pub trait JsValue: Debug {
    fn as_v8<'a>(&mut self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value>;
}
