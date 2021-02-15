pub use im::hashmap::*;
use im::*;
use intertrait::cast::*;
use intertrait::*;
use std::{any::*, fmt::*, result::*, sync::*};

use super::class::*;
use super::object::*;

// type PersistentHashMap = HashMap<Object, Object>;

pub struct PHashMap {
    inner: HashMap<Object, Object>,
}

impl IObject for HashMap<Object, Object> {
    fn get_class<'a>(&self) -> &'a Class {
        todo!()
    }

    fn call(&self, name: &str, obj: &Object, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: &str, obj: &Object) -> Object {
        todo!()
    }

    fn to_string(&self, obj: &Object) -> String {
        todo!()
    }

    fn get_hash(&self, obj: &Object) -> usize {
        todo!()
    }
}

castable_to!(PHashMap => IObject);

impl IObject for PHashMap {
    fn get_class<'a>(&self) -> &'a Class {
        todo!()
    }

    fn call(&self, name: &str, obj: &Object, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: &str, obj: &Object) -> Object {
        todo!()
    }

    fn to_string(&self, obj: &Object) -> String {
        todo!()
    }

    fn get_hash(&self, obj: &Object) -> usize {
        todo!()
    }
}

impl PHashMap {
    pub fn new(inner: HashMap<Object, Object>) -> PHashMap {
        PHashMap { inner }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
    }
}

static mut INIT: bool = false;
