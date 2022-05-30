fn main() {
    println!("Hello, world!");

    another_function(420);
    print_many_things(420, 'y')
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {} ", x)
}

fn print_many_things(x: i32, y: char) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
