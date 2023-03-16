pub fn variables(){
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant variable [THREE_HOURS_IN_SECONDS] = {} ", THREE_HOURS_IN_SECONDS);

    let immutable_int: i128 = 1234567890;
    println!("Int variable i = {}", immutable_int);
    let mut mutable_int: u128 = 1234567890;
    println!("Int variable i = {}", mutable_int);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Cast variable from string to int {}", guess);

    println!("Variable shadowing");
    shadowing();
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}