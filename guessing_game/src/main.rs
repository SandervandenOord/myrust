use rand::Rng;

fn play_guess_a_number() {
    println!("Please guess a number!");
    let random_number = rand::thread_rng().gen_range(0..=100);
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

    return ()

}

fn get_user_input() -> i32 {
    let mut user_input = String::new();
    let _ = std::io::stdin().read_line(&mut user_input);
    let user_input_trimmed = user_input.trim();
    println!("You guessed: {}", user_input_trimmed);
    let user_number = user_input_trimmed.parse::<i32>().unwrap();
    return user_number;
}

fn main() {
    let keep_playing = true;
    while keep_playing {
        play_guess_a_number();
    }

}
