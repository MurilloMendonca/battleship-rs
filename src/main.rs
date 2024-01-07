mod battleship;
use battleship::battleship::Game;

fn main() {
    let mut game = Game::new();
    game.print_map();

    while !game.is_game_over() {
        println!("Enter shot coordinates (e.g., a1):");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Some((x, y)) = parse_and_validate_input(&input) {
            if game.process_shot(x, y) {
                println!("Hit!");
            } else {
                println!("Miss!");
            }
            game.print_map();
        } else {
            println!("Invalid input. Please enter coordinates in the format 'a1'.");
        }
    }
    println!("Game over!");
    game.reveal_map();
}

fn parse_and_validate_input(input: &str) -> Option<(usize, usize)> {
    let input = input.trim().to_lowercase();
    let mut chars = input.chars();

    if let (Some(x_char), Some(y_char)) = (chars.next(), chars.next()) {
        if x_char.is_alphabetic() && y_char.is_digit(10) && chars.next().is_none() {
            let x = x_char as usize - 'a' as usize;
            let y = y_char.to_digit(10).unwrap() as usize - 1;

            if x < battleship::battleship::MAP_SIZE && y < battleship::battleship::MAP_SIZE {
                return Some((x, y));
            }
        }
    }

    None
}

