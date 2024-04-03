fn main() {
    println!("Hello, world!");

    another_function(10, 'u');

    let z = {
        let a = 3;
        a + 1
    };

    println!("Value from expression is {z}.");
    println!("Return value from function is {:?}.", return_function(5));
}

fn another_function(x: i32, y: char) {
    println!("Printing from another function.");
    println!("Integer passed to function is {x}.");
    println!("Character passed to funciton is {y}.");
}

fn return_function(x: i32) -> i32 {
    x + 1
}
