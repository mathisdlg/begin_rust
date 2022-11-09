use std::fmt; // Import `fmt`

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}


fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {x}");

    let mut space = " "; // mut -> allow to change the var in other space without let but if it's the same type
    println!("space: '{}'", space);
    space = "n";
    println!("The value of space is {space}");


    let space2 = " ";
    println!("{}", space2);
    let space2 = "n";
    println!("The value of space is {space2}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {spaces}");
    
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("One million is written as {}", 1_000_000u32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // But long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: {:?}", matrix); //Matrix: Matrix(1.1, 1.2, 2.1, 2.2)
    println!("Matrix: {}", matrix); //Matrix: (1.1, 1.2, 2.1, 2.2) (impl fmt::Display for Matrix)

    println!("Nombre avec 5 espaces: {:5?}", 5u8)
}