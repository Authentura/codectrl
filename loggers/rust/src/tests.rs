#![allow(dead_code)]
#![cfg(feature = "full")]

#[test]
fn test_backtrace() { test_backtrace_layer_2(); }

fn test_backtrace_final_layer() {
    let a = || crate::Log::log("Hello", Some(2), None);
    let _ = a();
}

fn test_backtrace_layer_2() { test_backtrace_layer_3(); }

fn test_backtrace_layer_3() { test_backtrace_final_layer(); }
