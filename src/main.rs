use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}


// References are immutable.
// References start with &. For instance `&foo` it's an immnutable reference, `&mut foo` it's a mutable reference.
// The syntax :: indicates an associated function, aka a static method. For instance `String::new();`

// `read_line` return an `io::Result` which is an enum that has two variants `Ok` and `Err`.
// `Result` has methods defined, one of them is `expected` which is going to make the app crash if `read_line` returned an `Err`.
//  Rust might complain if you don't use the `Result` value returned because that indicated that you aren't handling a possible error.

// `println!` it's calling the function with macros.
// the `{}` is a placehoder for a given value.