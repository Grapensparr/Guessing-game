use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

const MAX_NUMBER: u32 = 100;

struct GameState {
  correct_number: u32,
  max_attempts: u32,
  attempts: u32,
}

fn main() {
  println!("Welcome to Guess the Number!");
  println!("You have to guess a number between 1 and {}.", MAX_NUMBER);

  let mut game_state = initialize_game();

  loop {
    println!(
      "Please input your guess (attempt {}/{}):",
      game_state.attempts + 1,
      game_state.max_attempts
    );

    let guess = match get_user_input() {
      Ok(num) => num,
      Err(_) => {
        println!("{}", "Invalid input. Please enter a number.".red());
        continue;
      }
    };

    if guess < 1 || guess > MAX_NUMBER {
      println!(
        "{}",
        format!(
          "The number is between 1 and {}. Please try again.",
          MAX_NUMBER
        )
        .red()
      );
      continue;
    }

    game_state.attempts += 1;

    match guess.cmp(&game_state.correct_number) {
      Ordering::Less => println!("{}", "Too small!".red()),
      Ordering::Greater => println!("{}", "Too big!".red()),
      Ordering::Equal => {
        println!("{}", "Correct! You win!".green());
        break;
      }
    }

    if game_state.attempts >= game_state.max_attempts {
      println!(
        "You've run out of guesses. The correct number was {}.",
        game_state.correct_number
      );
      break;
    }
  }

  if play_again() {
    main();
  } else {
    println!("Thanks for playing!");
  }
}

fn initialize_game() -> GameState {
  let correct_number = rand::thread_rng().gen_range(1, MAX_NUMBER + 1);
  println!("The correct number is: {}", correct_number);

  GameState {
    correct_number,
    max_attempts: 5,
    attempts: 0,
  }
}

fn get_user_input() -> Result<u32, std::io::Error> {
  let mut guess = String::new();
  io::stdin().read_line(&mut guess)?;
  let parsed_guess: Result<u32, _> = guess.trim().parse();
  parsed_guess.map_err(|_| {
    io::Error::new(
      io::ErrorKind::InvalidInput,
      "Invalid input. Please enter a valid number.".to_string(),
    )
  })
}

fn play_again() -> bool {
  loop {
    println!("Do you want to play again? (Yes/No)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    match response.trim().to_lowercase().as_str() {
      "yes" => return true,
      "no" => return false,
      _ => {
        println!("{}", "Invalid response. Please enter 'Yes' or 'No'.".red());
        continue;
      }
    }
  }
}