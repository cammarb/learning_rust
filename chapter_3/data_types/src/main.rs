fn main() {
    // we need to add the type annotation for it to work
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // SCALAR TYPES: represents a single value (integers, floating-point, booleans & characters)

    // floating-point numbers
    let x = 2.0;
    println!("{}", x);
    let y: f32 = 3.0;
    println!("{}", y);

    let t = true;
    let f: bool = false;
    println!("t is: {}, and f is:{}", t, f);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
