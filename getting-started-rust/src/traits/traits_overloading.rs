pub mod traits_overloading {
    pub trait Foo {
        fn foo(self);
    }

    pub(crate) fn foo(x: impl Foo) {
        Foo::foo(x)
    }

    impl Foo for i32 {
        fn foo(self) {
            println!("This is int: {self}")
        }
    }

    impl Foo for f64 {
        fn foo(self) {
            println!("This is double: {self}")
        }
    }

    impl Foo for &str {
        fn foo(self) {
            println!("This is &str: {self}")
        }
    }
}