//! clojure::rust::class: Define class of objects

use std::sync::Arc;

use super::keywords::*;
use super::object::*;
use super::phashmap::*;

/// Class descriptor for Class :
/// ``` clojure
/// {
///     :super_class    Class
///     :protocols      [
///                        ... Protocols
///                     ]
///     :members        {
///                         :name            :clojure.rust.class/Class
///                         ... added from super-class
///                     }
///     :methods        {
///                         ... added from super-class
///                         ... added from protocols
///                     }
/// }
/// ```
pub struct Class {
    pub super_class: Object, // Class
    pub protocols: Object,   // HashSet of Protocols
    pub getters: Object,     // HashMap of Getters
    pub methods: Object,     // HashMap of Methods
    pub functions: Object,   // HashMap of static functions
}

/// trait for Class getters
trait IClass {
    fn super_class(&self) -> Object;
    fn protocols(&self) -> Object;
    fn getters(&self) -> Object;
    fn methods(&self) -> Object;
    fn functions(&self) -> Object;
}

/// getters of Class
impl IClass for Class {
    fn super_class(&self) -> Object {
        self.super_class.clone()
    }

    fn protocols(&self) -> Object {
        self.protocols.clone()
    }

    fn getters(&self) -> Object {
        self.getters.clone()
    }

    fn methods(&self) -> Object {
        self.methods.clone()
    }

    fn functions(&self) -> Object {
        self.functions.clone()
    }
}

impl Class {
    pub fn new(
        super_class: Object,
        protocols: Object,
        getters: Object,
        methods: Object,
        functions: Object,
    ) -> Object {
        Object {
            inner: Some(Arc::new(Class {
                super_class,
                protocols,
                getters,
                methods,
                functions,
            })),
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
    fn getter(&self, name: &str, obj: &Object) -> Object {
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
