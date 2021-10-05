use rusty_v8 as v8;

use std::mem::transmute;
use deno_core::OpPayload;
use deno_core::PromiseId;
use deno_core::error::type_error;
use deno_core::error::AnyError;
use serde::de::DeserializeOwned;
use crate::function::Function;

pub struct AccessibleOpPayload<'a, 'b, 'c> {
    pub(crate) scope: &'a mut v8::HandleScope<'b>,
    pub(crate) a: v8::Local<'c, v8::Value>,
    pub(crate) b: v8::Local<'c, v8::Value>,
    pub(crate) promise_id: PromiseId,
}
  
fn access_payload<'a, 'b, 'c, 'd>(payload: OpPayload::<'a, 'b, 'c>) -> AccessibleOpPayload::<'a, 'b, 'c> {
    let payload = unsafe {
        transmute::<OpPayload::<'a, 'b, 'c>, AccessibleOpPayload::<'a, 'b, 'c>>(payload)
    };

    payload
}

enum DeserializedValue<'a> {
    Function {
        function: Function<'a>,
    },
    Serde {
        value: DeserializeOwned,
    }
}

impl<'a, 'b, 'c> AccessibleOpPayload<'a, 'b, 'c> {
    pub fn deserialize<T: DeserializeOwned, U: DeserializeOwned>(
        self,
    ) -> Result<(T, U), AnyError> {
      let a: T = if self.a.is_function() {
          Function {
              scope: self.scope,
              v8_value: v8::Local::<v8::Function>::try_from(self.a).unwrap(),
          }
      } else {
        serde_v8::from_v8(self.scope, self.a)
            .map_err(AnyError::from)
            .map_err(|e| type_error(format!("Error parsing args: {}", e)))?
      };
  
      let b: U = serde_v8::from_v8(self.scope, self.b)
        .map_err(AnyError::from)
        .map_err(|e| type_error(format!("Error parsing args: {}", e)))?;
      Ok((a, b))
    }
  }