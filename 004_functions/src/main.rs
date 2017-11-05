fn main() {
    print_hello_world();
    print_value(10);
    add_numbers(8,5);
    let r = add(3,4);
    println!("Result of addition is: {}", r);

    let r = add_wrong(3,4);
}

fn print_hello_world(){
    println!("Hello, world!");
}

fn print_value(x: i32){
    println!("Value of variable is: {}", x);
}

fn add_numbers(a: i32, b: i32){
    println!("{} + {} = {}", a, b, a + b);
}

// right implementation of add function - it ends by expression and returns value
fn add(a: i32, b: i32) -> i32{
    a + b
}

// wrong implementation of add function - it ends by statement,
// so it doesn`t return value
fn add_wrong(a: i32, b: i32) -> i32{
    a + b;
}



