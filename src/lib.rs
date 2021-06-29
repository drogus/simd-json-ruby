//#[cfg(not(target_env = "msvc"))]
//use jemallocator::Jemalloc;
//
//#[cfg(not(target_env = "msvc"))]
//#[global_allocator]
//static GLOBAL: Jemalloc = Jemalloc;

#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, VM, Hash, NilClass, Fixnum, Float, Array, AnyObject};

use simd_json;
use serde_json::Value;

class!(SimdJsonRust);

fn convert_to_hash(json: &Value) -> Option<AnyObject> {
    match json.clone() {
        Value::Object(object) => { Some(value_to_object(json)) },
        _ => None,
    }
}

// {"foo": String("bar")}

fn value_to_object(json: &Value) -> AnyObject {
    match json {
        Value::Object(map) => {
            let mut hash = Hash::new();
            for (s, v) in map.iter() {
                let rstring = RString::new_utf8(&s.clone());
                hash.store(rstring, value_to_object(v));
            }
            hash.into()
        }
        Value::String(s) => RString::new(s).into(),
        Value::Null => NilClass::new().into(),
        Value::Array(vec) => {
            let mut rarray = Array::new();
            for v in vec.into_iter() {
                rarray.push(value_to_object(&v));
            }
            rarray.into()
        },
        Value::Number(n) => {
            if n.is_i64() {
                match n.as_i64() {
                    Some(n) => { Fixnum::new(n).into() },
                    None => unreachable!()
                }
            } else if n.is_u64() {
                match n.as_u64() {
                    Some(n) => { Fixnum::new(n as i64).into() },
                    None => unreachable!()
                }
            } else {
                match n.as_f64() {
                    Some(n) => { Float::new(n).into() },
                    None => unreachable!()
                }
            }
        },
        _ => RString::new("new").into(),
  
    }
}

methods!(
    SimdJsonRust,
    _rtself,

    fn parse(input: RString) -> AnyObject {
        let ruby_string = input.
          map_err(|e| VM::raise_ex(e) ).
          unwrap();

        let mut b = ruby_string.to_bytes_unchecked().to_vec();
        let v: Value = simd_json::serde::from_slice(&mut b).unwrap();
        let mut hash = Hash::new();
        let o = convert_to_hash(&v).unwrap();
        o
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