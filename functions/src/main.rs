fn main() {
    println!("Hello, World!");
    another_function(90);
}

// We can add parameters to functions when specifying their type
// It is compulsory to declare the type of any given parameter
fn another_function(x: i32) {
    println!("The value of x is : {}", increment(89));
}

// A perfectly valid function is this:

fn five() -> i32 {
    5 // No semicolon because we WANT to return this value
}

// All it does is returns the value

fn increment(x: i32) -> i32 {
    x + 1
}