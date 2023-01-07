// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn coordinate() -> (i32, i32) {
    (2, 10)
}

fn main() {
    // * Destructure the return value into two variables
    let (_, y) = coordinate();

    // * Use an if..else if..else block to determine what to print
    if y == 5 {
        println!("Equal to 5");
    } else if y > 5 {
        println!("greater than 5");
    } else {
        println!("less than 5");
    }
}

// fn main() {
//     let coord = (2, 3);
//     println!("{:?}, {:?}", coord.0, coord.1);

//     let (x, y) = (2, 3);
//     println!("{:?}, {:?}", x, y);

//     let (name, age) = ("Emma", 20);
//     println!("{:?}, {:?}", name, age);
// }
