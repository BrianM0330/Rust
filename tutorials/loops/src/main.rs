fn main() {
    for_loop_example();
    while_loop_exammple();
    loop_thru_collection_while();
    loop_thru_collection_for();
}

fn for_loop_example() {
    let mut counter: i32 = 0;
    println!("Print me first: {}", counter);

    let should_be_one: i32 = loop {
        println!("Pre increment: {}", counter);

        counter = counter + 1;
        println!("Post increment: {}", counter);

        if counter == 12 {
            break counter / 12;
        }
    };

    println!("This should be one: {}", should_be_one)
}

fn while_loop_exammple() {
    let mut number = 3;

    while number != 0 {
        println!("Hit the loop..");

        number -= 1;

        println!("Number is now: {}", number);
    }
}

fn loop_thru_collection_while () {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("Iterating through {:?}", a);

    while index < 5 {
        println!("The value of the array `a` at {} is: {}", index, a[index]);
        index += 1;
    }
}

fn loop_thru_collection_for () {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}