//! # object: Defines Rust dynamic Objects.
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

use std::{any::*, fmt::Debug, result::*, sync::*};

// use std::fmt::*;
// use intertrait::*;
// use intertrait::cast::*;

// use super::class::*;
// use super::rust_obj::*;

trait DynTrait: Any + Debug + 'static + Send + Sync {}

pub struct Object {
    pub content: Arc<dyn Any + 'static + Send + Sync>,
}

trait IObject {
    fn get_class(&self) -> &Object;
    fn get_super(&self) -> &Object;
    fn call(&self, name: &str) -> &Object;
    fn get_member(&self, name: &str) -> &Object;
    fn to_string(&self) -> String;
    fn get_hash(&self) -> usize;
}

impl Object {
    pub fn new<T>(content: Arc<T>) -> Object where
    T: DynTrait, 
    { 
        Object {
            content: content.clone(),
        }
    }

    pub fn get<T>(&self) -> Option<&T> where 
    T: Any + 'static + Send + Sync, 
    {
        // Return reference of pointed object
        self.content.downcast_ref::<T>()
    }
    pub fn get_mut<T>(&self) -> Option<&mut T> where 
    T: Any + 'static + Send + Sync, 
    {
        // Return reference of pointed object
        self.content.downcast_mut::<T>()
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.content)
    }

    pub unsafe fn init() {
        if INIT {return;}
        INIT = true;

        // Insures all is initialized
        // Class::init();
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(self.content, f)        
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            content: self.content.clone(),
        }
    }
}

static mut INIT: bool = false;
