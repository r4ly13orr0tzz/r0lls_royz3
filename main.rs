use guesser::*;
use std::io;
fn main(){
    println!("Guess the animal's child!");
    println!("Try it: "); // https://clck.ru/32mRdq
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");
    let jj = guesser::Rights{
        child: guess,
    };
    match guesser::guess_me(&jj){
        true => println!("You win!\n{:?}", guesser::get_it()),
        false => println!("You lose!\n{:?}", guesser::hint()),
    };
}