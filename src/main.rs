//number-guesser
//1. Generate a random number between 1-100
//2. Take input 
//3. if the num matches-> finish
//4. else-> too high or too low -> next input -> loop
use rand::Rng;
fn main() {
    rules();
    loop{
        let num = get_rand_num_0_100();
        let mut chances = 0;
        let mut guess = input();
        chances+=1;
        while guess != num {
            println!("{}", if guess>num {"too high"}else{"too low"});
            guess = input();
            chances+=1;
        }
        println!("Congrats! The number was: {} and you find it in {} chances", num, chances);
        if!play_again(){
            println!("Thanks for playing");
            break;
        }
    }
}
fn rules(){
    //displays the rules for playing
}
fn get_rand_num_0_100()->i32{
    rand::thread_rng().gen_range(0..101)
}
fn input()->i32{
    56
}
fn play_again()->bool{
    false
}

