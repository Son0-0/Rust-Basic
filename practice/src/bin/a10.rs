// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:

// * Use a function to print the messages
fn print_message(is_big: bool) {
    // * Use a match expression to determine which message
    //   to print
    match is_big {
        true => println!("its big"),
        false => println!("its small"),
    }
}

// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
fn main() {
    let num = 101;
    let is_big = num > 100;

    print_message(is_big);
}

// Example
// enum Access {
//     Admin,
//     Manager,
//     User,
//     Guest,
// }

// fn main() {
//     // secret file: admins only
//     let access_level = Access::Guest;
//     let can_accessfile = match access_level {
//         Access::Admin => true,
//         _ => false,
//     };

//     println!("can access: {:?}", can_accessfile);
// }
