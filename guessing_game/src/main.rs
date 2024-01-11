use rand::Rng;

fn play_guess_a_number() -> bool {
    println!("Please guess a number from 1 to 100!");

    let random_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("The number is: {}", random_number);
    let mut user_number = get_user_input();

    while user_number != random_number {
        if user_number < random_number {
            println!("The secret number is larger!")
        } else {
            println!("The secret number is smaller!")
        }
        user_number = get_user_input();
    }

    println!("You guessed it!");
    println!("Would you like to keep playing?");

    let keep_playing: bool = false;
    let check_playing = continue_playing();
    println!("{check_playing}");

    return keep_playing;
}

// fn continue_playing() -> String {
//     return String::from("y");
// }

fn continue_playing() -> &'static str {
    // &'static str is needed for returning a string literal
    "y"
}

fn get_user_input() -> i32 {
    let mut user_input = String::new();
    let _ = std::io::stdin().read_line(&mut user_input);
    println!("You guessed: {}", user_input);
    let user_number: i32 = user_input.trim().parse().unwrap();
    user_number
}

fn main() {
    loop {
        play_guess_a_number();
    }
}
