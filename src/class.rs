//! clojure::rust::class: Define class of objects

use super::object::Object;
use super::keywords::*;
use super::hashmap::HashMap;

pub struct Class {
    super_class:    Object, // Class
    protocols:      Object, // HashSet of Protocols
    members:        Object, // HashMap of Members
    methods:        Object, // HashMap of Methods
    functions:      Object, // HashMap of static functions
}

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

impl Class {
    pub fn new(name: String, map: Object) -> Object {
        let ob = 
                Class {
                    map:        map.clone(),
                };
        Object::new::<Class>(Keywords::get(name, *CORE),
                             Arc::new(ob)).clone()
    }
    
    /// Initialize all objects needed to create the Class interface
    pub unsafe fn init() {
        // only execute one time
        if INIT {return;}
        INIT = true;
        
        println!("Class::init");

        // Insures all is initialized
        Keywords::init();
        Object::init();
        HashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");

    }
}

static INIT: bool = false;
