//! # Defines Rust dynamic Objects.
//!
//! Define immutable dynamic objects
//!

#![allow(non_snake_case)]

use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::class::*;
// use super::rust_obj::*;

pub trait Inner: IObject + Debug + Eq + Hash + CastFromSync {}

/// Basic definition of object inner link to real structure
// pub type Inner = IObject;

/// Basic definition of a dynamic object
// pub type Object = Option<Arc<Inner>>;
pub struct Object {
    pub inner: Option<Arc<IObject>>,
}

castable_to!(Object => IObject);

impl Object {
    pub fn new<T: Inner>(obj: T) -> Object {
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
///
///
pub trait IObject: CastFromSync {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a Class;

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: &str, obj: &Object, args: &[Object]) -> Object;

    /// Call getter for a named `member`
    fn get(&self, name: &str, obj: &Object) -> Object;

    /// Return string representation of
    fn to_string(&self, obj: &Object) -> String;

    fn get_hash(&self, obj: &Object) -> usize;

    // fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //     f.write_str(&self.to_string(self)
    // }
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

    fn get(&self, name: &str, obj: &Object) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                a.get_class().get(name, obj)
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

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self;
        f.write_str(&self.to_string(self))
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

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

static mut INIT: bool = false;
