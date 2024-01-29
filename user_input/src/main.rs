use std::io;

fn main() {
    println!("Write down some text");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to Read line");
    println!("{}", input)
}