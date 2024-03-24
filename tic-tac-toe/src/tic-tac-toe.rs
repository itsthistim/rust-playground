use std::io::stdin;
use std::process::Command;

fn main() {
    let mut board = vec![' '; 9];
    const PLAYER1: char = 'X';
    const PLAYER2: char = 'O';

    let mut current_player = PLAYER1;

    loop {
        print_board(&board);
        println!("Player {}, enter your move (0-8):", current_player);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let index: usize = match input.trim().parse() {
            Ok(num) if num <= 8 => num,
            _ => {
                println!("Invalid input, try again.");
                continue;
            }
        };

        if board[index] != ' ' {
            println!("Invalid move, try again.");
            continue;
        }

        board[index] = current_player;

        if let Some(winner) = check_winner(&board) {
            print_board(&board);
            println!("Player {} wins!", winner);

            wait_for_input();
            break;
        }

        if board.iter().all(|&x| x != ' ') {
            print_board(&board);
            println!("It's a draw!");

            wait_for_input();
            break;
        }

        current_player = if current_player == PLAYER1 { PLAYER2 } else { PLAYER1 };
    }
}

fn print_board(board: &Vec<char>) {
    clear_console();
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---|---|---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---|---|---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn check_winner(board: &Vec<char>) -> Option<char> {
    for i in 0..3 {
        // check columns
        if board[i] != ' ' && board[i] == board[i + 3] && board[i] == board[i + 6] {
            return Some(board[i]);
        }

        // check rows
        let j = i * 3;
        if board[j] != ' ' && board[j] == board[j + 1] && board[j] == board[j + 2] {
            return Some(board[j]);
        }
    }

    // check diagonals
    if board[0] != ' ' && board[0] == board[4] && board[0] == board[8] {
        return Some(board[0]);
    }
    if board[2] != ' ' && board[2] == board[4] && board[2] == board[6] {
        return Some(board[2]);
    }

    None
}

fn wait_for_input() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
