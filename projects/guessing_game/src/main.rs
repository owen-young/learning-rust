use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    // random isn't part of the Rust standard library yet, so it must be in
    // a crate.

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
