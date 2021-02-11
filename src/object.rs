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
use std::clone::Clone;
use std::{any::*, fmt::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::null::*;
// use super::rust_obj::*;

pub type Object = Arc<IObject + 'static>;

/// `IObject` `Protocol` for all defined `Object`s
///
///
pub trait IObject {
    /// Return `Class` of `Object`
    unsafe fn get_class(&self) -> Object;

    /// Call named `method` with `Object`s arguments
    unsafe fn call(&self, name: &str, args: &[Object]) -> Object;

    /// Call getter for a named `member`
    unsafe fn getter(&self, name: &str) -> Object;

    /// Return string representation of
    unsafe fn to_string(&self) -> String;

    unsafe fn get_hash(&self) -> usize;
}

/// Implementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl IObject for Object {
    unsafe fn get_class(&self) -> Object {
        self.clone().get_class()
    }

    unsafe fn call(&self, name: &str, args: &[Object]) -> Object {
        self.get_class().call(name, args)
    }

    unsafe fn getter(&self, name: &str) -> Object {
        self.get_class().getter(name)
    }

    unsafe fn to_string(&self) -> String {
        self.get_class().to_string()
    }

    unsafe fn get_hash(&self) -> usize {
        self.get_class().get_hash()
    }
}
