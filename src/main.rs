use std::io;

fn main() {
    println!(">>>>>>>>>>>>>>>>>> Guessa >>>>>>>>>>>>>>>>>>> \n\n\n");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to readline");

    println!("You guessed: {} ", guess);
}
