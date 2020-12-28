
use super::object;

struct Null {}

impl IObject for Null {

}

impl Null {
    fn new() -> Object {
        Object::<Null>::new(Arc::new(Null {}))
    }
}

