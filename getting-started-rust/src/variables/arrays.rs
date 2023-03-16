pub fn array_demo() {
    let arr = [1,2,3,4,5];
    println!("Fixed array size: {}", arr.len());

    println!("Iterate array");
    for x in arr {
        println!("Array value = {}", x);
    }
}