//mod ops;
//mod function;

use std::any::Any;
use rusty_v8 as v8;

use std::mem::transmute;
use deno_core::PromiseId;
use deno_core::OpPayload;
use deno_core::OpState;
use std::cell::RefCell;
use deno_core::serialize_op_result;
use deno_core::Op;
use std::rc::Rc;
use deno_core::FsModuleLoader;
use deno_core::ModuleId;
use deno_core::ModuleSpecifier;
use deno_core::resolve_path;
use fruity_ecs::world::world::World;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

pub async fn execute_script(world: &mut World, path: &str) {
    /*let my_ext = Extension::builder()
    .middleware(|name, opfn| match name {
      "op_print" => deno_core::void_op_sync(),
      _ => opfn,
    })
    .build();*/

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![/*my_ext*/],
    module_loader: Some(Rc::new(FsModuleLoader {})),
    ..Default::default()
  });
  
  /*runtime.register_op(
    "op_sum",
    // The op-layer automatically deserializes inputs
    // and serializes the returned Result & value
    Box::new(move |state: Rc<RefCell<OpState>>, mut payload: OpPayload| -> Op {
      let result = payload.deserialize::<Vec<f64>, Value>().unwrap();

      let state_2 = state.borrow_mut();
      let payload_2 = get_scope_from_payload(&mut payload);

      let context = v8::Context::new(payload_2.scope);
      let global = context.global(payload_2.scope);

      let recv: v8::Local<v8::Value> = global.into();
      let _arg1 = v8::String::new(payload_2.scope, "un texte").unwrap();
      let _arg2 = v8::Integer::new(payload_2.scope, 2);

      
      let callback = v8::Local::<v8::Function>::try_from(result.1.v8_value).unwrap();
      callback.call(payload_2.scope, recv, &[]);

      let result = Ok(result.0.iter().sum::<f64>());
        
      Op::Sync(serialize_op_result(result, state))
    })
  );*/

  runtime.register_op(
    "world",
    // The op-layer automatically deserializes inputs
    // and serializes the returned Result & value
    Box::new(move |state: Rc<RefCell<OpState>>, payload: OpPayload| -> Op {
      let result = payload
        .deserialize::<Vec<f64>, Function>()
        .and_then(|(a, mut b)| {
          b.call();
          Ok(a.iter().sum::<f64>())
        });
        
      Op::Sync(serialize_op_result(result, state))
    }),
  );

  runtime.sync_ops_cache();

  let specifier: ModuleSpecifier = if let Ok(specifier) = resolve_path(path) {
    specifier
  } else {
      panic!("Problem in specifier creation");
  };

  let module_id: ModuleId = match runtime.load_main_module(&specifier, None).await {
      Ok(module_id) => module_id,
      Err(err) => panic!("Problem in load main module {}", err)
  };

  let mut receiver = runtime.mod_evaluate(module_id);
  receiver.close();
}

pub struct AccessibleOpPayload<'e, 'f, 'g> {
  pub(crate) scope: &'e mut v8::HandleScope<'f>,
  pub(crate) a: v8::Local<'g, v8::Value>,
  pub(crate) b: v8::Local<'g, v8::Value>,
  pub(crate) promise_id: PromiseId,
}

fn get_scope_from_payload<'a, 'b, 'c, 'd>(payload: &'d mut OpPayload::<'a, 'b, 'c>) -> &'d mut AccessibleOpPayload::<'a, 'b, 'c> {
  let payload = unsafe {
    transmute::<&'d OpPayload::<'a, 'b, 'c>, &'d mut AccessibleOpPayload::<'a, 'b, 'c>>(payload)
  };

  payload
}