use tarantool_rust_module::{BoxFunctionCtx, box_tuple_format_default, box_tuple_new, box_return_tuple, box_space_id_by_name, BOX_ID_NIL, box_replace, BoxTuple, box_error_set, box_tuple_ref, box_tuple_unref, box_error_last, BoxErrorype, box_error_message};
use serde::Serialize;
use std::os::raw::c_char;
use std::{slice, fmt};
use std::error::Error;
use std::ffi::CStr;
use std::convert::TryFrom;
//use syn::proc_macro::TokenStream;
//use syn::{parse_macro_input, parse_quote, Expr, Ident, ItemFn, Local, Pat, Stmt, Token};


//type Tuple = BoxTuple;
//impl Tuple {
//
//}

//#[proc_macro_attribute]
//pub fn tarantool_procedure(args: TokenStream, input: TokenStream) -> TokenStream {
//    let input = parse_macro_input!(input as ItemFn);
////    let x = format!(r#"
////            fn dummy() {{
////                println!("entering");
////                println!("args tokens: {{}}", {args});
////                println!("input tokens: {{}}", {input});
////                println!("exiting");
////            }}
////        "#,
////        args = args.into_iter().count(),
////        input = input.into_iter().count(),
////    );
////
////    x.parse().expect("Generated invalid tokens")
//}

pub struct BoxCtx {
    _ctx: *mut BoxFunctionCtx,
    _args_begin: *const c_char,
    _args_end: *const c_char,
}

#[derive(Debug, Clone)]
pub struct BoxError {
    error_type: &'static str,
    error_message: &'static str,
}
impl fmt::Display for BoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.error_message)
    }
}
impl Error for BoxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn get_last_error() -> BoxError {
    let box_error = unsafe {
        box_error_last()
    };

    let error_type = unsafe {
        let c_str = BoxErrorype(box_error);
        CStr::from_ptr(c_str).to_str().unwrap()
    };

    let error_message = unsafe {
        let c_str = box_error_message(box_error);
        CStr::from_ptr(c_str).to_str().unwrap()
    };

    BoxError{error_type, error_message}
}

impl BoxCtx {
    pub fn new(_ctx: *mut BoxFunctionCtx, _args_begin: *const c_char, _args_end: *const c_char) -> BoxCtx {
        BoxCtx { _ctx, _args_begin, _args_end }
    }

    pub fn respond<T>(&self, payload: &T) -> Result<(), Box<dyn Error>>
        where T: Serialize + ?Sized
    {
//        let mut buffer: [u8; 4096] = [0; 4096];
//        let writer = &mut buffer.as_mut();
//        rmp_serde::encode::write(writer, payload).unwrap();
        let buffer = rmp_serde::to_vec(payload)?;

        let tuple = unsafe {
            let fmt = box_tuple_format_default();
            let end = buffer.as_ptr().offset(buffer.len() as isize);
            box_tuple_new(fmt, buffer.as_ptr() as *const i8, end as *const i8)
        };
        assert!(!tuple.is_null());

        unsafe {
            box_tuple_ref(tuple);
            let res = box_return_tuple(self._ctx, tuple);
            box_tuple_unref(tuple);

            if res != 0 {
                return Err(Box::try_from(get_last_error()).unwrap())
            }

            assert_eq!(res, 0);
        };

        Ok(())
    }

    pub fn get_read_ref(&self) -> &[u8] {
        unsafe {
            let args_len = self._args_end as usize - self._args_begin as usize;
            &mut slice::from_raw_parts(self._args_begin as *const u8, args_len)
        }
    }
}

pub fn replace<T>(space_name: &str, tuple: &T) -> Result<(), Box<dyn Error>>
    where T: Serialize + ?Sized
{
    let space_id = unsafe {
        box_space_id_by_name(space_name.as_bytes().as_ptr() as *const c_char,
                             space_name.len() as u32)
    };
    assert_ne!(space_id, BOX_ID_NIL);

    println!("space id: {}", space_id);

    let buffer = rmp_serde::to_vec(tuple).unwrap();
    let buffer_begin = buffer.as_ptr();
    let buffer_end = unsafe { buffer_begin.offset(buffer.len() as isize) };



    unsafe {
        let res = box_replace(space_id, buffer_begin.cast(), buffer_end.cast(), 0 as *mut *mut BoxTuple);
        assert_eq!(res, 0);
    };

    Ok(())
}

pub fn proc_error(file: &str, line: u32, message: &str) {
    const ER_PROC_C: u32 = 102;
    unsafe {
        box_error_set(file.as_ptr() as *const c_char, line, ER_PROC_C, "%s".as_ptr() as *const c_char, message);
    };
}

