//! clojure::rust::class: Define class of objects

use intertrait::cast::*;
use intertrait::*;
use std::sync::Arc;
use std::{any::*, fmt::*, result::*, sync::*};

use super::keywords::*;
use super::object::*;
use super::phashmap::*;

/// ## Clojure Class descriptor for Class :
/// ``` clojure
/// {
///     :super_class    Class
///     :protocols      [
///                        ... Protocols
///                     ]
///     :get            {
///                         :name            :clojure.rust.class/Class
///                         ... added from super-class
///                     }
///     :methods        {
///                         ... added from super-class
///                         ... added from protocols
///                     }
/// }
/// ```
///
/// ## Rust Class descriptor for Class :
/// ``` rust
/// pub struct Class {
///     pub super_class: Object, // Class
///     pub protocols: Object,   // HashSet of Protocols
///     pub get: Object,         // HashMap of Getters
///     pub methods: Object,     // HashMap of Methods
///     pub functions: Object,   // HashMap of static functions
/// }
/// ```
pub struct Class {
    inner: PHashMap,
}

castable_to!(Class => IObject);

impl Class {
    pub fn new(inner: PHashMap) -> Object {
        Object {
            inner: Some(Arc::new(Class { inner })),
        }
    }

    /// Initialize all objects needed to create the Class interface
    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        println!("Class::init");

        // Insures all is initialized
        Keywords::init();
        Object::init();
        PHashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

impl IObject for Class {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a Class {
        todo!()
    }

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: &str, obj: &Object, args: &[Object]) -> Object {
        Object::null()
    }

    /// Call getter for a named `member`
    fn get(&self, name: &str, obj: &Object) -> Object {
        Object::null()
    }

    /// Return string representation of
    fn to_string(&self, obj: &Object) -> String {
        String::from("class")
    }

    fn get_hash(&self, obj: &Object) -> usize {
        0
    }
}

static mut INIT: bool = false;
