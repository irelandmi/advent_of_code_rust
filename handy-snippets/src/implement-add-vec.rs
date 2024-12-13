
struct Point(i32, i32);

// Implement subtraction for (i32, i32) tuples
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        (self.0 - other.0, self.1 - other.1)
    }
}

// Implement addition for (i32, i32) tuples
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        (self.0 + other.0, self.1 + other.1)
    }
}


fn main() {
    let a = Point(10, 20);
    let b = Point(5, 15);
    let c = a + b; // Works now!
    println!("{:?}", c); // Output: Point(15, 35)
}