// statement
pub fn add_numbers(x: i32, y: i32) -> i32 {
    println!("Sum is {}", x + y);
    x + y
}

pub fn expression() {
    // expression
    let number = {
        let x = 1;
        x + 2
    };

    println!("{}", number);
}
