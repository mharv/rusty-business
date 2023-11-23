use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Ownership and Borrowing
    let owner_string = String::from("Hello, ownership!");
    let modified_string = modify_and_return(owner_string);
    // println!("\nOwnership and Borrowing Result: {}", owner_string);
    println!("\nOwnership and Borrowing Result: {}", modified_string);

    // Lifetime System
    let string1 = String::from("Hello");
    let string2 = String::from("world!");
    let result = combine_strings(&string1, &string2, "Rust");

    println!("\nLifetime System Result: {}", result);
    println!("\nStill available for use: {}", string1);
    println!("\nStill available for use: {}", string2);

    // let string1 = String::from("Hello");
    // let result;
    // {
    //     let string2 = String::from("world!");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // Concurrency without Data Races
    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!(
    //     "Concurrency without Data Races Result: {}",
    //     *counter.lock().unwrap()
    // );

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for i in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         // Introduce a delay to slow down the execution
    //         // thread::sleep(std::time::Duration::from_secs(1));

    //         *num += 1;

    //         let thread_id = thread::current().id();

    //         // Print the thread ID holding the mutex
    //         println!("Thread {:?} acquired the lock", thread_id);
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!(
    //     "Concurrency without Data Races Result: {}",
    //     *counter.lock().unwrap()
    // );

    // Trait System and Pattern Matching
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // Using the draw_shape function to draw different shapes
    println!("\nTrait System and Pattern Matching Result: ");
    circle.reveal();
    rectangle.reveal();
    draw_shape(&circle);
    draw_shape(&rectangle);
    print_area(&circle);
    print_area(&rectangle);

    // No Null or Undefined Behavior
    // let optional_value: Option<i32> = Some(42);
    let optional_value: Option<i32> = None;

    println!("\nNo Null or Undefined Behavior Result: ");
    match optional_value {
        Some(value) => println!("No Null Result: {}", value),
        None => println!("No value"),
    }

    // Zero-Cost Abstractions
    let numbers: Vec<i32> = (1..=5).collect();
    let squared_numbers: Vec<i32> = square_numbers(numbers);
    println!("\nZero-Cost Abstractions Result: {:?}", squared_numbers);
}

// Updated definition of the Drawable trait
trait Drawable {
    fn draw(&self);
    fn area(&self);
}

// Implementing the Drawable trait for Circle
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }

    fn area(&self) {
        println!(
            "The area of this circle:  {}",
            std::f64::consts::PI * self.radius * self.radius
        );
    }
}

impl Circle {
    fn reveal(&self) {
        println!("{:?}", self);
    }
}

// Implementing the Drawable trait for Rectangle
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing a rectangle with width {} and height {}",
            self.width, self.height
        );
    }

    fn area(&self) {
        println!("The area of this rectangle:  {}", self.width * self.height);
    }
}

impl Rectangle {
    fn reveal(&self) {
        println!("{:?}", self);
    }
}

// Function demonstrating Trait System and Pattern Matching
fn draw_shape(shape: &dyn Drawable) {
    // Pattern matching based on trait methods
    shape.draw();
}

fn print_area(shape: &dyn Drawable) {
    // Pattern matching based on trait methods
    shape.area();
}

// Functions for each concept:

fn modify_and_return(s: String) -> String {
    s + " Modified"
}

fn combine_strings<'a, 'b, 'c>(s1: &'a str, s2: &'b str, s3: &'c str) -> String {
    format!("{} {} {}", s1, s2, s3)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn square_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().map(|x| x * x).collect()
}
