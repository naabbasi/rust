use log::{info, warn};

#[path = "variables/variables_demo.rs"]
mod variable_demo;
#[path = "variables/arrays.rs"]
mod array_demo;
#[path = "math_lib/math_lib.rs"]
mod math_lib;

#[path = "db/postgresql/helloworld_postgresql.rs"]
mod helloworld_postgresql;

fn main() {
    info!("Hello, world!");
    variable_demo::variables();
    array_demo::array_demo();
    info!("Add of 1 and 2 is : {}", math_lib::math::add(1, 2));
    info!("Add of 1 and 2 is : {}", math_lib::calc::add(1, 2));
    info!("Add of 1 and 2 is : {}", math_lib::calc::add(1, 2));
    warn!("Add of 1 and 2 is : {}", math_lib::calc::add(1, 2));
    helloworld_postgresql::db_postgresql::helloworld();
    helloworld_postgresql::db_postgresql::test().expect("Unable to perform db operations");
}
