fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10;        // Modify x through y
    println!("x = {}", x); // Prints 10
    // Correct way to create an immutable reference to the value of x:
    let z = x;    //z is now an immutable copy of x
    println!("z = {}", z); // Prints 10
}
