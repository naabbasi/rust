use log::{info, warn};
use crate::math_generic::math_generic::{Point};

#[path = "variables/variables_demo.rs"]
mod variable_demo;
#[path = "variables/arrays.rs"]
mod array_demo;
#[path = "math_lib/math_lib.rs"]
mod math_lib;

#[path = "math_lib/math_generic.rs"]
mod math_generic;

#[path = "variables/traits/traits_overloading.rs"]
mod traits_overloading;

#[path = "db/postgresql/helloworld_postgresql.rs"]
mod helloworld_postgresql;

fn main() {
    info!("Hello, world!");
    variable_demo::variables();
    array_demo::array_demo();
    println!("Add of 1 and 2 is : {}", math_lib::math::add(1, 2));
    println!("Subtract of 1 and 2 is : {}", math_lib::math::sub(1, 2));
    println!("Multiple of 1 and 2 is : {}", math_lib::math::mul(1, 2));

    let divide = math_lib::math::divide(2, 0);
    if divide.is_err() {
        println!("{}", divide.err().unwrap())
    } else {
        println!("Divide of 1 and 2 is : {}", divide.unwrap());
    }

    let divide = math_lib::math::divide(2, 2);
    if divide.is_err() {
        println!("{}", divide.err().unwrap())
    } else {
        println!("Divide of 1 and 2 is : {}", divide.unwrap());
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    traits_overloading::traits_overloading::foo(1);
    traits_overloading::traits_overloading::foo(11.1);
    traits_overloading::traits_overloading::foo("string");

    helloworld_postgresql::db_postgresql::helloworld();
    //helloworld_postgresql::db_postgresql::test().expect("Unable to perform db operations");
}
