use rand::prelude::*;

fn aimove() -> String {
    let mut rng = thread_rng();
    let moves = ["rock", "paper", "scissors"];
    return moves[rng.gen_range(0..3)].to_string().to_lowercase();
}

fn wincon(comp: &String, player: &String) -> String {
    if comp == player || comp == "scissors" && player == "scissor" {
        return "tie".to_string();
    }
    if comp == "rock" && player == "scissors" || comp == "rock" && player == "scissor"{
        return "comp".to_string();
    }
    if comp == "paper" && player == "rock" {
        return "comp".to_string();
    }
    if comp == "scissors" && player == "paper" {
        return "comp".to_string();
    }
    return "player".to_string();
}

pub fn run() {
    println!("Rock Paper Scissors");
    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;
    loop {
        println!("Enter your move (exit to quit): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let playermove = input.trim().to_string().to_lowercase();
        if playermove == "exit" {
            break;
        } else if playermove == "stats" {
            println!("Wins: {}\nLosses: {}\nTies: {}", wins, losses, ties);
            continue;
        } else if playermove != "rock" && playermove != "paper" && playermove != "scissors" && playermove != "scissor" {
            println!("Invalid move... Valid moves are rock, paper, scissors or stats.");
            continue;
        }
        let aimove = aimove();
        let res = wincon(&aimove.to_owned(), &playermove);
        if res == "player" {
            println!("You win! I chose {}.", aimove);
            wins += 1;
        } else if res == "comp" {
            println!("You lose! I chose {}.", aimove);
            losses += 1;
        } else {
            println!("It's a tie!");
            ties += 1;
        }
    }
}