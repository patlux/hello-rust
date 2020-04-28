use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Hey Patrick!");
  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Expected a guess");
    print!("You guessed: {}", guess);

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    let re = guess.cmp(&secret_number);

    match re {
      Ordering::Equal => {
        println!("Correct!");
        break;
      }
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
    }
  }
}
