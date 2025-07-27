fn main() {

    // declaring constants
    const PI: f32 = 3.14;
    println!("The value of the constant pi id {PI}");

    // scalar data types

    /*
     * integere data types are:
     * i8, i16, i32, i64, i128, isize -> signed integers
     * u8, u16, u32, u64, u128, usize -> unsigned integers
     */
     let _x: isize = 45;

     /*
      * floating point data types are:
      * f32, f64 -> floating point numbers
      */
      let _y: f64 = 3.14;

      // boolean data type
      let _z: bool = true;

      // char data type
      let _c: char = 'a';

      // compound data types

      // typle
      let _tuple: (i32, f64, bool) = (42, 3.14, true); // fixed legth cant grow onmce declared

      let tup = (500, true, 46.9);

      // destructuring tuple
      let (a, b, c) = tup;
      println!("the value of a is {a}, b is {b}, and c is {c}");

      // another way of assesing tuple is
      println!("the value of a is {}, b is {}, and c is {}", tup.0, tup.1, tup.2);

      // arrays
      // should have all the values of same data type unlike tuple
      let _a = [1, 2, 3, 4, 5];
      let _a: [i32; 5] = [1, 2, 3, 4, 5];
      let _a = [3; 5]; // a = [3, 3, 3, 3, 3];


      // function calling
      new_function();

      // calling add function
      let result = add(5, 3);
      println!("The result of adding 5 and 3 is {result}");

      //calling multiply function
      let result = multiply(5, 3);
      println!("The result of multiplying 5 and 3 is {result}");

      // loops
      control_flow();
}

fn new_function() {
    println!("hello rust, how do you do!")
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn control_flow() {

    // declare an infinite loop
    loop {
        println!("Hello, world!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    let mut count = 0;
    'counting_up: loop {    // tagging a loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    demonstrate_for();
}

fn demonstrate_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
