//! clojure::rust::object: Defines the Rust equvalent of Java Objects.
#![allow(non_snake_case)]

use std::{any::Any, mem::transmute, sync::*};
use std::fmt::*;
use intertrait::*;
use intertrait::cast::*;

pub use crate::clojure;
use crate::class::*;
use crate::rust_obj::*;

#[derive(Debug)]
struct Object {
    content: Arc<dyn Any>,
}

struct Null {}

castable_to!(Null => [sync] Object);

trait IObject {
    fn get_class(self: Arc<Self>) -> Arc<Self>;
    fn call(self: Arc<Self>) -> Self;
    fn get(&self, name: &str) -> Self;
    fn is_class(&self, class: &str) -> bool;
    fn is_protocol(&self, protocol: &str) -> bool;
    fn to_string(&self) -> String;
    fn get_hash(&self) -> usize;
}

castable_to!(Data => [sync] Object, Debug);

impl IObject for Null {

}

impl Object {
    pub fn new<T>(content: Arc<T>) -> Object {
        Object {
            content: content.clone(),
        }
    }

    pub fn get<T>(&self) -> Arc<T> {
        unsafe {
            // Return reference of pointed object
            transmute::<usize, Arc<T>>(self.content)
        }
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.content)
    }

    pub unsafe fn init() {
        if INIT {return;}
        INIT = true;

        // Insures all is initialized
        Class::init();
    }
}

static mut INIT: bool = false;


impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            content: self.content.clone(),
        }
    }
}
