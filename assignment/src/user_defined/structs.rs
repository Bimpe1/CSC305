//Exercise
/*
I need similar implementation for Circle and Triangle
Besides Area, I need Perimeter and comparison on the basis of Perimeter
In your submission, I need a comment against every line of code about what it is meant to achieve
 */

// This suppresses warnings when a given declared function is not used.
#![allow(dead_code)]
#[warn(unused_variables)]
// Import the 'Ordering' enum for comparison
use std::cmp::Ordering;

// Define the 'Shape' trait
trait Shape {
    // Associated function to create a new instance with a radius and name
    fn new(radius: f32, name: &'static str) -> Self;
    // Associated function to create a new instance with sides and name
    fn new2(side1: f32, side2: f32, side3: f32, name: &'static str) -> Self;
    // Method to calculate the area of the shape
    fn area(&self) -> f32;
    // Method to calculate the perimeter of the shape
    fn perimeter(&self) -> f32;
    // Method to set the radius of the shape
    fn set_radius(&mut self, radius: f32);
    // Method to get the radius of the shape
    fn get_radius(&self) -> f32;
    // Method to set the name of the shape
    fn set_name(&mut self, name: &'static str);
    // Method to get the name of the shape
    fn get_name(&self) -> &str;
}

// Implement the 'Circle' struct
#[derive(Default, Debug, Clone)]
struct Circle {
    radius: f32,
    name: &'static str,
}
impl Circle {
    // Default constructor for Circle with default values
    fn default() -> Self {
        Circle {
            radius: 1.0,
            name: "default_name",
        }
    }
}

// Implement the 'Shape' trait for 'Circle'
impl Shape for Circle {
    // Constructor for Circle with given radius and name
    fn new(radius: f32, name: &'static str) -> Self {
        Circle { radius, name }
    }

    // Method to calculate the area of the circle
    fn area(&self) -> f32 {
        std::f32::consts::PI * (self.radius as f32).powi(2)
    }

    // Method to calculate the perimeter of the circle
    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius as f32
    }

    // Method to set the radius of the circle
    fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    // Method to get the radius of the circle
    fn get_radius(&self) -> f32 {
        self.radius
    }

    // Method to set the name of the circle
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    // Method to get the name of the circle
    fn get_name(&self) -> &str {
        self.name
    }

    // Implementing the 'new' associated function for creating a new instance of the struct.
    // The 'todo!' macro is a placeholder that indicates this function needs to be implemented.
    fn new2(_side1: f32, _side2: f32, _side3: f32, _name: &'static str) -> Self {
        todo!()
    }
}

// Implement comparison traits for 'Circle'
impl PartialEq for Circle {
    // Method to check if two circles are equal based on perimeter
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter()
    }

    // Method to check if two circles are not equal based on perimeter
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implementation of PartialOrd trait for 'Circle'
impl PartialOrd for Circle {
    // Method to compare two circles based on perimeter
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
}

// Implement the 'Triangle' struct
#[derive(Default, Debug, Clone)]
struct Triangle {
    side1: f32,
    side2: f32,
    side3: f32,
    name: &'static str,
}
impl Triangle {
    // Default constructor for Triangle with default values
    fn default() -> Self {
        Triangle {
            side1: 1.0,
            side2: 1.0,
            side3: 1.0,
            name: "default_name",
        }
    }
}

// Implement the 'Shape' trait for 'Triangle'
impl Shape for Triangle {
    // Constructor for Triangle with given sides and name
    fn new2(side1: f32, side2: f32, side3: f32, name: &'static str) -> Self {
        Triangle {
            side1,
            side2,
            side3,
            name,
        }
    }

    // Method to calculate the area of the triangle
    fn area(&self) -> f32 {
        // Heron's formula for the area of a triangle
        if self.side1 + self.side2 > self.side3
            && self.side1 + self.side3 > self.side2
            && self.side2 + self.side3 > self.side1
        {
            let s = (self.side1 + self.side2 + self.side3) as f32 / 2.0;
            (s * (s - self.side1 as f32) * (s - self.side2 as f32) * (s - self.side3 as f32)).sqrt()
        } else {
            // Invalid triangle, return 0 or handle it as appropriate for your use case
            0.0
        }
    }

    // Method to calculate the perimeter of the triangle
    fn perimeter(&self) -> f32 {
        self.side1 + self.side2 + self.side3
    }

    // Method to set the radius of the triangle (no-op for triangles)
    fn set_radius(&mut self, _radius: f32) {
        // Triangles don't have a radius, so this is a no-op
    }

    // Method to get the radius of the triangle (always returns 0 for triangles)
    fn get_radius(&self) -> f32 {
        0.0
    }

    // Method to set the name of the triangle
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    // Method to get the name of the triangle
    fn get_name(&self) -> &str {
        self.name
    }

    // Implementing the 'new' associated function for creating a new instance of the struct.
    // This function takes a radius (f32) and a name (static string) as parameters.
    // The 'todo!' macro is a placeholder that indicates this function needs to be implemented.
    fn new(_radius: f32, _name: &'static str) -> Self {
        todo!()
    }
}

// Implement comparison traits for 'Triangle'
impl PartialEq for Triangle {
    // Method to check if two triangles are equal based on perimeter
    fn eq(&self, other: &Self) -> bool {
         self.perimeter() == other.perimeter()
    }

    // Method to check if two triangles are not equal based on perimeter
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implementation of PartialOrd trait for 'Triangle'
impl PartialOrd for Triangle {
    // Method to compare two triangles based on perimeter
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.area().partial_cmp(&other.area()) {
            Some(Ordering::Equal) => self.perimeter().partial_cmp(&other.perimeter()),
            result => result,
        }
    }
}

// Entry point of the program
pub fn run() {
    // Create a default circle
    let circle = Circle::default();

    // Display circle properties
    println!("Default");
    println!("");
    println!("Radius: {}", circle.get_radius());
    println!("Name: {}", circle.get_name());
    println!("Area: {}", circle.area());
    println!("Perimeter: {}", circle.perimeter());

    // Create a circle with specific values
    let circle2 = Circle::new(5.0, "Circle2");

    // Display circle2 properties
    println!("Radius of circle2: {}", circle2.get_radius());
    println!("Name: {}", circle2.get_name());
    println!("Area: {}", circle2.area());
    println!("Perimeter: {}", circle2.perimeter());

    // Compare circles using PartialOrd
    let result1 = circle.partial_cmp(&circle2);
    println!("Result1 = {:?}", result1);

    // Compare circles using PartialEq
    let result2 = circle.eq(&circle2);
    println!("Result2 = {:?}", result2);

    let result3 = circle.ne(&circle2);
    println!("Result3 = {:?}", result3);

    println!("");
    println!("Triangle");
    println!("");

    // Create a default triangle
    let triangle = Triangle::default();

    // Display triangle properties
    println!("Side1: {}", triangle.side1);
    println!("Side2: {}", triangle.side2);
    println!("Side3: {}", triangle.side3);
    println!("Name: {}", triangle.name);

    // Create a triangle with specific values
    let triangle2 = Triangle::new2(10.0, 20.0, 20.0, "Triangle2");

    // Display triangle2 properties
    println!("Side1: {}", triangle2.side1);
    println!("Side2: {}", triangle2.side2);
    println!("Side3: {}", triangle2.side3);
    println!("Name: {}", triangle2.name);
    println!("Area: {}", triangle2.area());
    println!("Perimeter: {}", triangle2.perimeter());

    // Compare triangles using PartialEq
    let result1 = triangle.eq(&triangle2);
    println!("Result1 = {:?}", result1);

    let result2 = triangle.ne(&triangle2);
    println!("Result2 = {:?}", result2);

    // Compare triangles using PartialOrd
    let result3 = triangle.partial_cmp(&triangle2);
    println!("Result3 = {:?}", result3);
}
