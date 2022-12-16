use std::{io::Stdin, num::ParseIntError};
use rand::{Rng, Error};
fn main(){
    let stdin = std::io::stdin();
    rules(&stdin);
    loop {
        let mut attempts = 0;
        let mut choosed_num = choose_a_num();
        println!("I have chosen a number between 1 and 100, now guess it...");
        let mut guessed_num=get_num(&stdin);
        attempts+=1;
        while guessed_num!=choosed_num {
            if guessed_num>choosed_num {
                println!("too high");
            }
            else{
                println!("too low");
            }
            guessed_num = get_num(&stdin);
            attempts+=1;
        }
        println!("Congrats. You guessed it right. In {} attempt(s)", attempts);
        if !play_again(&stdin){
            println!("Thanks for playing...");
            break;
        }
    }
}

fn rules(stdin:&Stdin){
    let mut ip = String::new();
    println!("***NUMBER GUESSER GAME***");
    println!("    ---BY SANDIPAN---    ");
    println!("Rules: Press enter");
    stdin.read_line(&mut ip);
    println!("1. I will think of a number between 1 and 100.");
    stdin.read_line(&mut ip);
    println!("2. You have to guess the number. Then type the number. And press enter.");
    stdin.read_line(&mut ip);
    println!("3. If you guesses right.. you win. Otherwise I will tell if the number is too hign or too low. Then you have to guess it again..");
    stdin.read_line(&mut ip);
    println!("4. This will continue until you guessed the right number.");
    stdin.read_line(&mut ip);
    println!("You are ready to play... Let's begin...");
}

fn choose_a_num()->u32{
    //returns a random number between 1 and 100, inclusive range
    rand::thread_rng().gen_range(1..101)
}
fn get_num(stdin:&Stdin)->u32{
    let mut s = String::new();
    stdin.read_line(&mut s);
    let pintR = s.trim().parse::<u32>();
    match &pintR {
        Ok(x) => {return *x;},
        Err(e) => {
            println!("Error: Your entry must be a number. Please Enter again.");
            return get_num(stdin);
        }
    }
}
fn play_again(stdin:&Stdin)->bool{
    println!("Do you want to play again? Press enter to continue. Press q to quit.");
    let mut x = String::new();
    stdin.read_line(&mut x);
    match x.trim() {
        "q" => false,
        _ => true,
    }
}