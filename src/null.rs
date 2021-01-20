//! Define Null object

use lazy_static::lazy_static;
use std::{fmt::*, result::*, sync::*};

use super::object::*;

/// Empty `Null` struct for null
pub struct Null {}

// castable_to!(Null => [sync] IObject, Debug);

// castable_to!(Object => IObject, Debug);

impl IObject for Null {
    fn get_class(&self) -> Object {
        NULL.clone()
    }

    fn call(&self, name: &str, args: &[Object]) -> Object {
        NULL.clone()
    }

    fn getter(&self, name: &str) -> Object {
        NULL.clone()
    }

    fn to_string(&self) -> String {
        String::from("Null")
    }

    fn get_hash(&self) -> usize {
        0
    }
}

impl Debug for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Null")
    }
}

impl Null {
    fn new() -> Object {
        Object::new::<Null>(Arc::new(Null {}))
    }

    pub unsafe fn init() {
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
    }
}

static mut NULL: Object = Null::new();
static mut INIT: bool = false;

#[test]
fn test_null_valid() {
    let a = Null::new();
    println!("Null values:");
    println!("------------");
    println!("to_string = {:?}", a.to_string());
    println!("get_hash  = {:?}", a.get_hash());
    println!("get_class = {:?}", a.get_class().to_string());
    println!("call      = {:?}", a.call("to_string", &[NULL]).to_string());
}

fn test_vect() {}
