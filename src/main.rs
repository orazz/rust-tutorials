use std::io;

mod conditions;
mod functions;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let x: i32 = input.trim().parse().unwrap();

    // println!("{}", int_input + 2);

    println!("{:?}", conditions::conditions());
    println!("{:?}", conditions::control_flow());

    println!("{:#?}", functions::add_numbers(x, x));
    println!("{:#?}", functions::expression());
}
