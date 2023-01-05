// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum Flavor {
    Sweet,
    Fruity,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("sweet"),
        Flavor::Fruity => println!("fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 99.99,
    };

    print_drink(sweet);

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 77.77,
    };

    print_drink(fruity);
}

// practice
// struct GroceryItem {
//     stock: i32,
//     price: f64, // == double
// }

// let cereal = GroceryItem {
//     stock: 10,
//     price: 2.99,
// };

// println!("stock: {:?}", cereal.stock);
// println!("price: {:?}", cereal.price);
