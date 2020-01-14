extern crate core;
extern crate rmp_serde;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde as rmps;

#[allow(dead_code)]
mod tarantool;

pub mod tnt_box;

use tnt_box::{
    BoxCtx,
    proc_error
};

use std::os::raw::{c_int, c_char};
use tarantool::{
    BoxFunctionCtx,
};
use std::error::Error;
use crate::tnt_box::replace;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Args<'a> {
    uuid: &'a str,
    some_str: &'a str,
    another_str: &'a str,
}

#[no_mangle]
pub extern fn rustproc(_ctx: *mut BoxFunctionCtx, _args_begin: *const c_char, _args_end: *const c_char) -> c_int {
    let ctx = BoxCtx::new(_ctx, _args_begin, _args_end);

    let reader = ctx.get_read_ref();
    let args: Args = rmp_serde::from_read_ref(reader).unwrap();
    match rust_procedure(&ctx, &args) {
        Ok(()) => (),
        Err(err) => proc_error(file!(), line!(), err.description()),
    }

    return 0;
}

fn rust_procedure(ctx: &BoxCtx, args: &Args) -> Result<(), Box<dyn Error>> {
    println!("{:?}", args);

    const SPACE_NAME: &'static str = "space";
    replace(SPACE_NAME, &args)?;
    ctx.respond("some string")?;

    Ok(())
}
