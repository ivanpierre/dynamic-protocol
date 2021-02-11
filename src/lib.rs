#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(bare_trait_objects)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod keywords;
pub mod null;
pub mod object;

// pub mod class;
// pub mod implementation;
// pub mod member;
// pub mod protocol;
// pub mod prototype;
// pub mod rust_obj;
// pub mod hashmap;
// pub mod hashset;
// pub mod vector;
// pub mod function;
