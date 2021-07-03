extern crate rutie;
use rutie::{Class, Object, RString, VM, Boolean, Hash, NilClass, Fixnum, Float, Array, AnyObject};

use simd_json;
use simd_json::{Node, StaticNode};

use std::time::{Duration, Instant};

fn tape_to_object(tape: &[Node]) -> AnyObject {
    // we get to this method with the first element in the tape
    // that is not null. we can either return it directly for
    // simple types or dig deeper with `node_to_object`
    let element = tape[0];
    match element {
        Node::Static(StaticNode::Null) => NilClass::new().into(),
        Node::Static(StaticNode::Bool(b)) => Boolean::new(b).into(),
        Node::Static(StaticNode::I64(i)) => Fixnum::new(i).into(),
        // TODO: this means that we handle u64 as i64, which seems wrong,
        // this should be probably handled by a Bignum, but it doesn't seem
        // implemented in rutie yet
        Node::Static(StaticNode::U64(u)) => Fixnum::new(u as i64).into(),
        Node::Static(StaticNode::F64(f)) => Float::new(f).into(),
        Node::String(s) => RString::new_utf8(s).into(),
        Node::Array(count, end) => {
            tape_to_ruby(&tape[0..(end-1)], 2, end)
        }
        Node::Object(count, end) => {
            tape_to_ruby(&tape[0..(end-1)], 2, end)
        },
    }
}

fn array_to_ruby(tape: &[Node], offset: usize, end: usize) -> AnyObject {
    let mut i = 0;
    let mut array = Array::new();
    while i < tape.len() {
        let value = match tape[i] {
            Node::Object(count, end) => {
                let result = object_to_ruby(&tape[(i+2)..(end - offset + 1)], offset + 2, end);
                i = end - offset;
                result
            },
            Node::Array(count, end) => {
                let result = array_to_ruby(&tape[(i+2)..(end - offset + 1)], offset + 2, end);
                i = end - offset;
                result
            },
            Node::Static(StaticNode::Null) => {
                i += 1;
                NilClass::new().into()
            },
            Node::Static(StaticNode::Bool(b)) => {
                i += 1;
                Boolean::new(b).into()
            },
            Node::Static(StaticNode::I64(integer)) => {
                i += 1;
                Fixnum::new(integer).into()
            },
            // TODO: this means that we handle u64 as i64, which seems wrong,
            // this should be probably handled by a Bignum, but it doesn't seem
            // implemented in rutie yet
            Node::Static(StaticNode::U64(u)) => {
                i += 1;
                Fixnum::new(u as i64).into()
            },
            Node::Static(StaticNode::F64(f)) => {
                i += 1;
                Float::new(f).into()
            }
            Node::String(s) => {
                i += 1;
                RString::new_utf8(s).into()
            },
        };

        array.push(value);
    }

    array.into()
}

fn object_to_ruby(tape: &[Node], offset: usize, end: usize) -> AnyObject {
    let mut i = 0;
    let mut hash = Hash::new();
    while i < tape.len() {
        println!("OBJECT: {:?}", tape);
        let key = match tape[i] {
            Node::String(s) => RString::new_utf8(s),
            _ => unreachable!(),
        };
        println!("I: {}, key: {}", i, key.to_str());
        let value = match tape[i+1] {
            Node::Object(count, end) => {
                let result = object_to_ruby(&tape[(i+2)..(end - offset + 2)], offset + 2, end);
                i = end - offset + 2;
                result
            },
            Node::Array(count, end) => {
                let result = array_to_ruby(&tape[(i+2)..(end - offset + 2)], offset + 2, end);
                i = end - offset + 2;
                result
            },
            Node::Static(StaticNode::Null) => {
                i += 2;
                NilClass::new().into()
            },
            Node::Static(StaticNode::Bool(b)) => {
                i += 2;
                Boolean::new(b).into()
            },
            Node::Static(StaticNode::I64(integer)) => {
                i += 2;
                Fixnum::new(integer).into()
            },
            // TODO: this means that we handle u64 as i64, which seems wrong,
            // this should be probably handled by a Bignum, but it doesn't seem
            // implemented in rutie yet
            Node::Static(StaticNode::U64(u)) => {
                i += 2;
                Fixnum::new(u as i64).into()
            },
            Node::Static(StaticNode::F64(f)) => {
                i += 2;
                Float::new(f).into()
            }
            Node::String(s) => {
                i += 2;
                RString::new_utf8(s).into()
            },
        };

        hash.store(key, value);
    }

    hash.into()
}

fn tape_to_ruby(tape: &[Node], offset: usize, end: usize) -> AnyObject {
    println!("tape_to_ruby, tape: {:?}, offset: {}, end: {}, tape.len: {}", tape, offset, end, tape.len());
    match tape[0] {
        Node::Array(count, end) => {
            let result = array_to_ruby(&tape[3..(end - offset + 1)], offset + 2, end);
            result
        },
        Node::Object(count, end) => {
            println!("PASSING OBJECT: {:?}, end: {}, offset: {}", tape, end, offset);
            let result = object_to_ruby(&tape[1..(end - offset + 1)], offset + 2, end);
            result
        },
        _ => unreachable!()
    }
}

pub fn foo() {

}

pub fn rust_parse(ruby_string: RString) -> AnyObject {
    let start = Instant::now();
    let mut b = ruby_string.to_bytes_unchecked().to_vec();
    println!("ruby_string.to_bytes_unchecked().to_vec(): {}us, {}ms", start.elapsed().as_micros(), start.elapsed().as_millis());
    let start = Instant::now();
    let tape: Vec<Node> = simd_json::to_tape(&mut b).unwrap();
    println!("simd_json::to_tape(): {}us, {}ms", start.elapsed().as_micros(), start.elapsed().as_millis());
    // it looks like tape always has a `Node(Static(Null))` as a first element,
    // I think we can skip it
    //println!("{:?}", tape);
    let start = Instant::now();
    let result = tape_to_object(&tape[1..]);
    println!("tape_to_object.to_vec(): {}us, {}ms", start.elapsed().as_micros(), start.elapsed().as_millis());
    result
}

