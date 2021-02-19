#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(bare_trait_objects)]

pub mod class;
pub mod globals;
pub mod object;
pub mod phashmap;
pub mod pvector;
pub mod rustobj;
pub mod unique;

// pub mod implementation;
// pub mod member;
// pub mod protocol;
// pub mod prototype;
// pub mod hashset;
// pub mod function;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
