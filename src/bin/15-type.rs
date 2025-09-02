// Define an enum with two operations
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Implement a method for the enum
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Create a type alias for convenience
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // Use the alias instead of the long name
    let add = Operations::Add;
    let sub = Operations::Subtract;

    let result1 = add.run(10, 5);
    let result2 = sub.run(10, 5);

    println!("10 + 5 = {}", result1);
    println!("10 - 5 = {}", result2);
}
