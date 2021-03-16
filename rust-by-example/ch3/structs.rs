#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// If this function did not have a borrow, then the rectangle would be totally 
// destroyed after using it. This can be shown if you remove the & from this signature
// and remove the one that passes it in on the line where this function is called. 
// doing a borrow completely negates that, and we can keep using stuff afterwards. 
fn rect_area(rectangle: &Rectangle) -> f32 {
    let Point { x: tx, y: ty } = rectangle.top_left;
    let Point { x: bx, y: by } = rectangle.bottom_right;
    (bx - tx) * (ty - by)
}

fn square(point: &Point, len: f32) -> Rectangle {
    let Point { x: bx, y: by } = point;
    Rectangle {
        top_left: Point { x: *bx, y: *by+len },
        bottom_right: Point { x: *bx+len, y: *by}
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one

    // this ".." syntax updates the values using the ones you specified, but keeps
    // the other ones. In this case, we keep the y value.
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("area of rectangle is: {}", rect_area(&_rectangle));
    // looks like extracting bottom_right and top_left did not destroy them. I think this is just because
    // bottom_right and top_left aren't bindings, they are just names of stuff. so they aren't destroyed
    // until the end of the function when the entire rectangle is.
    println!("using stuff after calling rect_area {:?}", _rectangle.bottom_right);

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("square is: {:?}", square(&point, 5.2));
}
