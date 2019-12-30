extern crate core;
extern crate rmp_serde;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde as rmps;

#[allow(dead_code)]
mod tarantool;

use std::os::raw::{c_int, c_char};
use std::slice;

use tarantool::{BoxFunctionCtx};
use crate::tarantool::{box_tuple_format_default, box_tuple_new, box_return_tuple};

#[derive(Debug, Deserialize, PartialEq)]
struct Args<'a> {
    some_str: &'a str,
    another_str: &'a str,
}

#[no_mangle]
pub extern fn rustproc(ctx: *mut BoxFunctionCtx, args_begin: *const u8, args_end: *const u8) -> c_int {
    assert!(!ctx.is_null());
    assert!(!args_begin.is_null());
    assert!(!args_end.is_null());

    let args_blob = unsafe {
        let args_len = args_end as usize - args_begin as usize;
        slice::from_raw_parts(args_begin, args_len)
    };

    let reader = &mut args_blob.as_ref();
    let args: Args = rmp_serde::from_read_ref(reader).unwrap();

    println!("{:?}", args);

    let buffer = rmp_serde::to_vec(&(args.some_str, args.another_str)).unwrap();
    let tuple = unsafe {
        let fmt = box_tuple_format_default();
        let end = buffer.as_ptr().offset(buffer.len() as isize);
        box_tuple_new(fmt, buffer.as_ptr() as *const i8, end as *const i8)
    };

    unsafe {
        box_return_tuple(ctx, tuple);
    };

    return 0;
}
