use rusty_v8 as v8;
use std::cell::UnsafeCell;

thread_local!(static THREAD_SCOPE_STACK: UnsafeCell<Vec<&'static mut v8::HandleScope<'static>>> = UnsafeCell::new(Vec::new()));

pub fn push_thread_scope_stack(scope: &mut v8::HandleScope) {
    THREAD_SCOPE_STACK.with(|scope_stack| {
        let scope_stack = unsafe { &mut *scope_stack.get() };
        let scope =
            unsafe { std::mem::transmute::<&mut v8::HandleScope, &mut v8::HandleScope>(scope) };
        scope_stack.push(scope);
    });
}

pub fn top_thread_scope_stack<'a>() -> Option<&'a mut v8::HandleScope<'a>> {
    let scope = THREAD_SCOPE_STACK.with(|scope_stack| {
        let scope_stack = unsafe { &mut *scope_stack.get() };
        scope_stack.last_mut()
    });

    scope.map(|scope| unsafe {
        std::mem::transmute::<&mut v8::HandleScope, &mut v8::HandleScope>(scope)
    })
}

pub fn pop_thread_scope_stack<'a>() -> Option<&'a mut v8::HandleScope<'a>> {
    let scope = THREAD_SCOPE_STACK.with(|scope_stack| {
        let scope_stack = unsafe { &mut *scope_stack.get() };
        scope_stack.pop()
    });

    scope.map(|scope| unsafe {
        std::mem::transmute::<&mut v8::HandleScope, &mut v8::HandleScope>(scope)
    })
}

pub fn clear_thread_scope_stack() {
    THREAD_SCOPE_STACK.with(|scope_stack| {
        let scope_stack = unsafe { &mut *scope_stack.get() };
        scope_stack.clear();
    });
}
