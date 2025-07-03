pub fn main() {
    let mut player_score = 0;
    let mut computer_score = 0;
    let rounds = 5;
    let choices = ["rock", "paper", "scissors"];
    
    println!("Hey, welcome to the 'Rock, Paper, Scissors' game!");
    
    // For _ in 0..rounds means we will play for a fixed number of rounds
    // is a variable that holds the number of rounds to play
    // _ is a placeholder for the loop variable since we don't need it
    
    // We will use the choices' array to randomly select the computer's choice
    // rand::random::<usize>() % choices.len() will give us a random index
    // from the choice array, which contains "rock", "paper", and "scissors"
    // The computer will randomly select one of these choices for each round
    // The player will input their choice, and we will compare the two choices
    for _ in 0..rounds {
        println!("Please enter your choice (rock, paper, scissors):");
        let mut player_choice = String::new();
        std::io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line!");
        
        // Trim whitespace and convert to lowercase for consistency
        // This ensures that the player's input is valid regardless of how they type it
        let player_choice = player_choice.trim().to_lowercase();
        
        // Check if the player's choice is valid
        // If the player enters something other than "rock", "paper", or "scissors",
        // we will prompt them to try again
        
        // player_choice.as_str() converts the String to a &str for comparison
        // choices.contains(&player_choice.as_str()) checks if the player's choice is in the choices array
        if !choices.contains(&player_choice.as_str()) {
            println!("Invalid choice! Please try again.");
            continue;
        }

        let computer_choice = choices[rand::random::<usize>() % choices.len()];
        println!("Computer chose: {}", computer_choice);

        if player_choice == computer_choice {
            println!("It's a tie!");
        } else if (player_choice == "rock" && computer_choice == "scissors") ||
                  (player_choice == "paper" && computer_choice == "rock") ||
                  (player_choice == "scissors" && computer_choice == "paper") {
            println!("You win this round!");
            player_score += 1;
        } else {
            println!("Computer wins this round!");
            computer_score += 1;
        }
    }
}