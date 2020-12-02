use std::io;

fn fibonacci(term : u32) -> u32 {
    match term {
        0 => 0,
        1 => 1,
        _ => fibonacci(term - 1) + fibonacci(term - 2)
    }
}

fn main() {
    println!("Please input the nth term for the Fibonacci sequence:");

    let mut term = String::new();

    io::stdin().read_line(&mut term).expect("Failed to read line");

    let term : i32 = term.trim().parse().expect("Please indicate a number!");

    let value = fibonacci(term.abs() as u32);

    println!("Fibonacci value at term {} is {}", term, value);
}
