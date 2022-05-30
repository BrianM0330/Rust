fn main() {
    let x = 69;
    let x = 1;
    println!("test: {}", x);
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    /* 
    Tuples
    Containers for multiple types
    */
    let tup: (i32, u32, char, char) = (-69, 69, '🦀', 'z');

    // destructuring a tuple
    let (x, y, z, z2) = tup;
    println!("The values of x, y, z, z2 are: {}, {}, {}, {}", x, y, z, z2);

    let sixty_nine_neg = tup.0;
    let sixty_nine = tup.1;
    let crab = tup.2;

    println!("The values of sixty_nine_neg, sixty_nine, crab: {}, {}, {}", sixty_nine_neg, sixty_nine, crab);

    /* 
        Arrays
        Containers for single types
    */
    

}