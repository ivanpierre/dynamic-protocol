//! Define Null object

// use intertrait::cast::*;
// use intertrait::*;

use lazy_static::lazy_static;
use std::{any::*, fmt::Debug, result::*, sync::*};

use super::object::*;

/// Empty `Null` struct for null
pub struct Null {}

// castable_to!(Object => IObject, Debug);

impl IObject for Null {
    fn get_class(&self) -> &Object {
        &NULL.clone()
    }

    fn call(&self, name: &str, args: &[Object]) -> &Object {
        &NULL.clone()
    }

    fn getter(&self, name: &str) -> &Object {
        &NULL.clone()
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
        Object::new(Arc::new(Null {}))
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

lazy_static! {
    pub static ref NULL: Object = Null::new();
}

static INIT: bool = false;
