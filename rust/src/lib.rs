extern crate core;
extern crate rmp_serde;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde as rmps;

#[allow(dead_code)]
mod tarantool;

use std::os::raw::{c_int, c_char};
use std::slice;
use tarantool::{
    BoxFunctionCtx,
    box_tuple_format_default,
    box_tuple_new,
    box_return_tuple,
    box_space_id_by_name,
    BOX_ID_NIL,
    box_replace,
    BoxTuple,
};
use serde::Serialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Args<'a> {
    uuid: &'a str,
    some_str: &'a str,
    another_str: &'a str,
}

#[no_mangle]
pub extern fn rustproc(ctx: *mut BoxFunctionCtx, args_begin: *const c_char, args_end: *const c_char) -> c_int {
    assert!(!ctx.is_null());
    assert!(!args_begin.is_null());
    assert!(!args_end.is_null());

    let reader = unsafe {
        let args_len = args_end as usize - args_begin as usize;
        &mut slice::from_raw_parts(args_begin as *const u8, args_len)
    };

    let args: Args = rmp_serde::from_read_ref(reader).unwrap();
    println!("{:?}", args);

    const SPACE_NAME: &str = "space";
    let space_id = unsafe {
        box_space_id_by_name(SPACE_NAME.as_bytes().as_ptr() as *const c_char,
                             SPACE_NAME.len() as u32)
    };
    assert_ne!(space_id, BOX_ID_NIL);

    println!("space id: {}", space_id);

    unsafe {
        let res = box_replace(space_id, args_begin, args_end, 0 as *mut *mut BoxTuple);
        assert_eq!(res, 0);
    };

    // serialize response to msgpack and return from procedure
    respond(ctx, &(args.some_str, args.another_str));

    return 0;
}

fn respond<T>(ctx: *mut BoxFunctionCtx, payload: &T) where T: Serialize + ?Sized {
    let buffer = rmp_serde::to_vec(payload).unwrap();
    let tuple = unsafe {
        let fmt = box_tuple_format_default();
        let end = buffer.as_ptr().offset(buffer.len() as isize);
        box_tuple_new(fmt, buffer.as_ptr() as *const i8, end as *const i8)
    };
    assert!(!tuple.is_null());

    unsafe {
        let res = box_return_tuple(ctx, tuple);
        assert_eq!(res, 0);
    };
}
