use rand::prelude::*;

fn aimove() -> String {
    let mut rng = thread_rng();
    let moves = ["rock", "paper", "scissors"];
    return moves[rng.gen_range(0..3)].to_string().to_lowercase();
}

fn wincon(comp: String, player: String) -> String {
    if comp == player {
        return "tie".to_string();
    }
    if comp == "rock" && player == "scissors" {
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
    loop {
        println!("Enter your move (exit to quit): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let playermove = input.trim().to_string().to_lowercase();
        if playermove == "exit" {
            break;
        } else if playermove != "rock" && playermove != "paper" && playermove != "scissors" {
            println!("Invalid move");
            continue;
        }
        let aimove = aimove();
        let res = wincon(aimove, playermove);
        if res == "player" {
            println!("You win!");
        } else if res == "comp" {
            println!("You lose!");
        } else {
            println!("It's a tie!");
        }
    }
}