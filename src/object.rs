//! # Defines Rust dynamic Objects.
//!
//! Define immutable dynamic objects
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

#![allow(non_snake_case)]

use intertrait::CastFromSync;
use lazy_static::lazy_static;
use std::{any::*, fmt::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::null::*;
// use super::rust_obj::*;

/// Dynamic `Object` structure
pub struct Object {
    /// The `content` is an `Arc` reference of an `Any` protocol (Say anything we use). This enable to avoid to have a limited and non-dynamic `Enum` to define polymorphism.
    pub content: Arc<dyn CastFromSync + 'static>,
}

// castable_to!(Object => [sync] IObject, Debug);

/// `IObject` `Protocol` for all defined `Object`s
///
///
pub trait IObject {
    /// Return `Class` of `Object`
    fn get_class(&self) -> Object;

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: &str, args: &[Object]) -> Object;

    /// Call getter for a named `member`
    fn getter(&self, name: &str) -> Object;

    /// Return string representation of `Object`
    fn to_string(&self) -> String;

    /// Return hashcode
    fn get_hash(&self) -> usize;
}

impl Object {
    /// Create new `object` of given Type
    pub fn new<T>(content: Arc<T>) -> Object
    where
        T: CastFromSync + 'static,
    {
        Object {
            content: content.clone(),
        }
    }

    /// Get hard count of `content`s Arc
    pub fn count(&self) -> usize {
        Arc::strong_count(&self.content)
    }

    /// Get IObject's version of `Object`
    pub fn get_object(obj: Arc<dyn CastFromSync>) -> Arc<dyn IObject> {
        match obj.clone().cast::<dyn IObject>() {
            Ok(res) => res.clone(),
            _ => panic!("ca va pas"),
        }
    }

    /// Get protocol's version of `Object`
    pub fn get<T>(self) -> &'static T {
        match self.content.clone().cast::<&'static T>() {
            Ok(res) => &*res,
            _ => panic!("ca va pas"),
        }
    }

    /// Initialize static data
    pub unsafe fn init() {
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        // Class::init();
    }
}

impl Debug for Object {
    /// Use Object's `content`'s format
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}

impl Clone for Object {
    /// clone `content` and create new object
    fn clone(&self) -> Self {
        Object {
            content: self.content.clone(),
        }
    }
}

/// Implementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl IObject for Object {
    fn get_class(&self) -> Object {
        Object::get_object(self.content.clone()).get_class()
    }

    fn call(&self, name: &str, args: &[Object]) -> Object {
        Object::get_object(self.content.clone()).call(name, args)
    }

    fn getter(&self, name: &str) -> Object {
        Object::get_object(self.content.clone()).getter(name)
    }

    fn to_string(&self) -> String {
        Object::get_object(self.content.clone()).to_string()
    }

    fn get_hash(&self) -> usize {
        Object::get_object(self.content.clone()).get_hash()
    }
}

static mut INIT: bool = false;
