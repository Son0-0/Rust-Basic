// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:

fn main() {
    // * Use a mutable integer variable
    let mut value: i32 = 1;
    // * Use a loop statement
    loop {
        // * Print the variable within the loop statement
        println!("{:?}", value);
        if value == 4 {
            // * Use break to exit the loop
            break;
        }
        value += 1;
    }
}

// fn practice() {
//     let mut cnt: i32 = 0;

//     loop {
//         if cnt == 5 {
//             break;
//         }
//         cnt += 1;
//         println!("hello {:?}!", cnt);
//     }
// }
