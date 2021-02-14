//! # Defines Rust dynamic Objects.
//!
//! Define immutable dynamic objects
//!

#![allow(non_snake_case)]

use intertrait::CastFromSync;
use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, fmt::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::class::*;
// use super::rust_obj::*;

/// Basic definition of object inner link to real structure
pub type Inner = IObject + 'static;

/// Basic definition of a dynamic object
// pub type Object = Option<Arc<Inner>>;
pub struct Object {
    pub inner: Option<Arc<Inner>>,
}

impl Object {
    pub fn new<T: IObject + 'static>(obj: T) -> Object {
        Object {
            inner: Some(Arc::new(obj)),
        }
    }

    pub fn null() -> Object {
        Object { inner: None }
    }

    pub fn is_null(&self) -> bool {
        match self.inner {
            None => true,
            Some(_) => false,
        }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }

        INIT = true;

        println!("Class::init");

        // Insures all is initialized
        Class::init();
    }
}

/// `IObject` `Protocol` for all defined `Object`s
///rachel maddow
///
pub trait IObject {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a Class;

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: &str, obj: &Object, args: &[Object]) -> Object;

    /// Call getter for a named `member`
    fn getter(&self, name: &str, obj: &Object) -> Object;

    /// Return string representation of
    fn to_string(&self, obj: &Object) -> String;

    fn get_hash(&self, obj: &Object) -> usize;
}

const NILSTRING: &str = "nil";

/// Implementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl IObject for Object {
    fn get_class<'a>(&self) -> &'a Class {
        if let Some(o) = self.clone().inner {
            o.get_class()
        } else {
            panic!("Get class from nil")
        }
    }

    fn call(&self, name: &str, obj: &Object, args: &[Object]) -> Object {
        match self.clone().inner {
            None => panic!("Call on nil"),
            Some(o) => {
                let a = o.clone();
                o.get_class().call(name, obj, args)
            }
        }
    }

    fn getter(&self, name: &str, obj: &Object) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                a.get_class().getter(name, obj)
            }
        }
    }

    fn to_string(&self, obj: &Object) -> String {
        match self.clone().inner {
            None => String::from(NILSTRING),
            Some(o) => {
                let a = o.clone();
                a.get_class().to_string(obj)
            }
        }
    }

    fn get_hash(&self, obj: &Object) -> usize {
        match self.clone().inner {
            None => 0,
            Some(o) => {
                let a = o.clone();
                a.get_class().get_hash(obj)
            }
        }
    }
}

impl Clone for Object {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }

    fn clone(&self) -> Self {
        let Object { inner } = self;
        match inner {
            None => Object { inner: None },
            Some(o) => Object {
                inner: Some(o.clone()),
            },
        }
    }
}

static mut INIT: bool = false;
