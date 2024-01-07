use rand::Rng;

fn play_guess_a_number() -> bool{
    println!("Please guess a number from 1 to 100!");
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("The number is: {}", random_number);
    let mut user_number = get_user_input();

    while user_number != random_number {
        if user_number < random_number {
            println!("The secret number is larger!")
        }
        else {
            println!("The secret number is smaller!")
        }
        user_number = get_user_input();

    }

    println!("You guessed it!");

    let keep_playing: bool = false;

    return keep_playing;

}

fn get_user_input() -> i32 {
    let mut user_input = String::new();
    let _ = std::io::stdin().read_line(&mut user_input);
    println!("You guessed: {}", user_input);
    let user_number = user_input.trim().parse::<i32>().unwrap();
    user_number
}

fn main() {
    let mut keep_playing = true;
    while keep_playing {
        keep_playing = play_guess_a_number();
    }
}
