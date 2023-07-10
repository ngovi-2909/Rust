fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
    exercise8();
}
use std::io;
fn exercise8() {
    let mut accounting = vec!["Alice", "Ben"];
    let mut add_input = String::new();
    loop {
        io::stdin()
            .read_line(&mut add_input.to_string())
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person);
    }
}