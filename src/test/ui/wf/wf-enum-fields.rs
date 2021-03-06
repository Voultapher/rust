// Test that we check struct fields for WFedness.

#![feature(associated_type_defaults)]

#![allow(dead_code)]

struct IsCopy<T:Copy> {
    value: T
}

enum SomeEnum<A> {
    SomeVariant(IsCopy<A>) //~ ERROR E0277
}


fn main() { }
