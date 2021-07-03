use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;#[macro_use]

extern crate rutie;
use rutie::{Class, Object, RString, VM, Boolean, Hash, NilClass, Fixnum, Float, Array, AnyObject};

mod implementation;

use crate::implementation::rust_parse;

class!(SimdJsonRust);

methods!(
    SimdJsonRust,
    _rtself,

    fn parse(input: RString) -> AnyObject {
        let ruby_string = input.
          map_err(|e| VM::raise_ex(e) ).
          unwrap();


        rust_parse(ruby_string)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_simd_json() {
    Class::new("SimdJsonRust", None).define(|klass| {
        klass.def_self("parse", parse);
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
