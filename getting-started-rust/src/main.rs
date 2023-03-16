#[path = "variables/variables_demo.rs"] mod variable_demo;
#[path = "variables/arrays.rs"] mod array_demo;
//mod calculator;

fn main() {
    println!("Hello, world!");
    variable_demo::variables();
    array_demo::array_demo();
}
