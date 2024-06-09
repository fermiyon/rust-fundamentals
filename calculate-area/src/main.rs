/// A trait for calculating the area of a shape.
trait Area {
    fn area(&self) -> Result<f64, &'static str>;
}

/// A struct representing a circle.
struct Circle {
    radius: f64,
}

/// A struct representing a square.
struct Square {
    length: f64,
}

/// Implement the Area trait for Circle.
impl Area for Circle {
    fn area(&self) -> Result<f64, &'static str> {
        if self.radius > 0.0 {
            Ok(std::f64::consts::PI * self.radius.powi(2))
        } else {
            Err("Invalid radius size")
        }
    }
}

/// Implement the Area trait for Square.
impl Area for Square {
    fn area(&self) -> Result<f64, &'static str> {
        if self.length > 0.0 {
            Ok(self.length.powi(2))
        } else {
            Err("Invalid length size")
        }
    }
}

/// An enum to represent different shapes.
enum Shape {
    Circle(Circle),
    Square(Square),
}

fn main() {
    let shapes = vec![
        Shape::Circle(Circle { radius: 5.0 }),
        Shape::Square(Square { length: 3.0 }),
        Shape::Circle(Circle { radius: 2.0 }),
    ];

    // The collect::<Result<Vec<_>, _>>() method attempts to collect the transformed
    // items into a Result containing a vector of areas. If any of the areas resulted in an error,
    // the entire collection will result in an Err
    //
    // The areas inside the map function, after collect, is a Vec<f64> only if the Result is Ok.
    // To sum up the elements of this vector, we need to turn it into an iterator first,
    // with the areas.iter(). The iter method borrows each element of the collection,
    // allowing to use them to calculate the sum without taking ownership of the vector.
    let total_area: Result<f64, &'static str> = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(circle) => circle.area(),
            Shape::Square(square) => square.area(),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|areas| areas.iter().sum()); // Sum the areas if they are all valid

    match total_area {
        Ok(area) => println!("Total area: {:.2} sq. units", area),
        Err(e) => eprintln!("Error calculating area: {}", e),
    }
}
