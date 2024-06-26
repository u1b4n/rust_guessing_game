use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(r#"Welcome to the "guessing game!"#);
    let random_number: u32 = rand::thread_rng().gen_range(0..=100);

    // while user hasn't figured the number out repeat
    loop {
        println!("Take a guess: ");
        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line.");

        // shadowing the variable to parse it from a String to an u32
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse your input to an unsigned integer.");
                // go to next loop iteration
                continue;
            }
        };

        match user_input.cmp(&random_number) {
            Ordering::Greater => println!("The number is to big."),
            Ordering::Less => println!("The number is to small."),
            Ordering::Equal => {
                println!("Nice you got the number. Program will exit in 2 seconds.");
                std::thread::sleep(std::time::Duration::from_millis(2000));
                return;
            }
        };
    }
}
