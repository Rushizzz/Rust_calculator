use std::io;

fn main() {
    let num1: f32;
    let num2: f32;
    let sign: char;

    let mut user_input = String::new();
    println!("Please enter first number:");
    let _ = io::stdin().read_line(&mut user_input).expect("Please enter number");
    num1 = user_input.trim().parse::<f32>().unwrap();

    let mut user_input = String::new();
    println!("Please enter Second number:");
    let _ = io::stdin().read_line(&mut user_input).expect("Please enter number");
    num2 = user_input.trim().parse::<f32>().unwrap();

    let mut user_input = String::new();
    println!("Please enter Operator:");
    let _ = io::stdin().read_line(&mut user_input).expect("Please enter number");
    sign = user_input.trim().chars().next().unwrap();

    println!("num1 = {}, num2 = {}, Sign: {}", num1, num2, sign);

    match sign {
        '+' => println!("{}", num1 + num2),
        '-' => println!("{}", num1 - num2),
        '*' => println!("{}", num1 * num2),
        '/' => println!("{}", num1 / num2),
        _ => { println!("Invalid sign"); return;},
    };
}
