fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y;    // z is an immutable reference to y
    *y = 10;        // Modify x through y
    println!("x = {}", x); // Prints 10 
    *z = 20;        // This will cause a compile time error 
                   // because z is immutable, but the code tries to modify x via z
}