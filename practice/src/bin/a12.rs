// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics

// * Use an enum for the box color
enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!(
            "width: {:?} height: {:?} depth: {:?}",
            self.width, self.height, self.depth
        );
    }
}

struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let small_box = ShippingBox::new(1.0, Color::Red, small_dimensions);
    small_box.print();

    let big_dimensions = Dimensions {
        width: 10.0,
        height: 20.0,
        depth: 30.0,
    };

    let big_box = ShippingBox::new(10.0, Color::Blue, big_dimensions);
    big_box.print();
}
