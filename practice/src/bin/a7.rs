// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:

enum Color {
    Red,
    Blue,
}

fn main() {
    // * Use an enum with color names as variants
    print_color(Color::Red);
    print_color(Color::Blue);
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(my_color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match my_color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
    }
}
