use std::io::Write;
use std::num::*;
use std::io;

fn user_input(prompt: &str) -> u32 {
    let mut input_text = String::new();
    loop {
        print!("{}", prompt.to_string());
        io::stdout().flush().expect("Failed to write to stdout!");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin!");
        match input_text.trim().parse::<u32>() {
            Err(..) => {println!("Invalid answer!"); continue},
            Ok(i) => return i,
        }
    }
}

fn main() {
    print!("<1> Convert (10) base to (-1 + i) base.\n");
    print!("<2> Convert (-1 + i) base to (10) base.\n");
    loop {
        match user_input("Choose conversion method <1-2>: ") {
            1 => real_to_imaginary(user_input("Enter number to convert: ")),
            2 => imaginary_to_real("x"),
            _ => continue,
        }
    }
}

fn real_to_imaginary(x: u32) {
    let const div = Complex::new(-1, 1);
}
fn imaginary_to_real(x: &str) {

}
