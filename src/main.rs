use std::io::Write;
use num_complex::Complex;
use std::io;

fn user_input_string(prompt: &str) -> String {
    let mut input_text = String::new();
    loop {
        print!("{}", prompt.to_string());
        io::stdout().flush().expect("Failed to write to stdout!");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin!");
        return input_text.trim().to_string();
    }
}

fn user_input(prompt: &str) -> i64 {
    let mut input_text = String::new();
    loop {
        print!("{}", prompt.to_string());
        io::stdout().flush().expect("Failed to write to stdout!");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin!");
        match input_text.trim().parse::<i64>() {
            Err(..) => {println!("Invalid answer!"); continue},
            Ok(i) => return i,
        }
    }
}

fn main() {
    println!("<1> Convert (10) base to (-1 + i) base.");
    println!("<2> Convert (-1 + i) base to (10) base.");
    loop {
        match user_input("Choose conversion method <1-2>: ") {
            1 => println!("Resulted (-1 + i) number: {}\n", real_to_imaginary(Complex::new(user_input("Enter real part of number: "), user_input("Enter imaginary part of number: ")))),
            2 => {
                match imaginary_to_real(user_input_string("Enter number to convert: ")) {
                    Ok(n) => println!("Resulted complex number: {} + {}i\n", n.re, n.im),
                    Err(err) => println!("{}\n", err),
                }
            },

            _ => {println!("Enter valid conversion method!"); continue},
        }
    }
}

fn real_to_imaginary(x: Complex<i64>) -> String {
    unimplemented!();
    const DIV = Complex::new(-1, 1);
    while x.re != 0 && x.im != 0 {

    }
}

fn imaginary_to_real(x: String) -> Result<Complex<i64>, String> {
    let mut ans: Complex<i64> = Complex::new(0, 0);
    let mut count = 0;
    for i in x.chars().rev() {
        match i {
            '1' => {
                ans += Complex::new(-1, 1).powu(count);
            },
            '0' => (),
            _ => return Err("Invalid number in base (-1 + i). Can use only 0 and 1!".to_string()),
        }
        count += 1;
    }
    return Ok(ans);
}
