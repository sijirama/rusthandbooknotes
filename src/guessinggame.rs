use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guessinggame() {

    println!("Guess the number");
    
    loop {
    
        println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess  = String::new();

        
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue
    };

    println!("You guessed : {guess}");
    println!("Secret number: {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too big"),
        Ordering::Less=> println!("Too small"),
        Ordering::Equal=>{
        println!("Yo win");
        break;
        }
    }
}
}
