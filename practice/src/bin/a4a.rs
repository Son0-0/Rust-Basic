// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:

fn main() {
    // * Use a variable set to either true or false
    let some_bool = true;

    // * Use a match expression to determine which message to display
    match some_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}

// fn practice() {
//     let some_bool = true;
//     match some_bool {
//         true => println!("its true"),
//         false => println!("its false"),
//     }

//     let some_int = 4;
//     match some_int {
//         1 => println!("its 1"),
//         2 => println!("its 2"),
//         3 => println!("its 3"),
//         _ => println!("its something else"),
//     }

//     let my_name = "Seungwoo";
//     match my_name {
//         "Seungwoo" => println!("that is my name"),
//         "Bob" => println!("not my name"),
//         "Alice" => println!("hello alice"),
//         _ => println!("nice to meet you!"),
//     }
// }
