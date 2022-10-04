mod test_base_crate {
    pub mod test_sub_mod;

//     pub use test_sub_mod::foo;
}

// use test_base_crate::test_sub_mod;
// use test_base_crate::test_sub_mod::foo;
//
mod test2;

fn main() {
    println!("Hello, world!");
    crate::test_base_crate::test_sub_mod::foo();
    // test_base_crate::test_sub_mod::foo();
    // test_sub_mod::foo();
    // foo();
    // crate::test_base_crate::foo();
    //
    test2::test3::foo()
}
