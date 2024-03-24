use std::io::stdin;
use std::process::Command;

const ANSI_COLOR_RESET: &str = "\x1b[0m";
const ANSI_COLOR_GRAY: &str = "\x1b[90m";
const ANSI_COLOR_YELLOW: &str = "\x1b[33m";

fn main() {
    let mut board = vec![' '; 9];
    const PLAYER1: char = 'X';
    const PLAYER2: char = 'O';

    let mut current_player = PLAYER1;

    loop {
        print_board(&board);

        println!("\nPlayer {}, enter your move (0-8): ", current_player);

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
            println!("\nPlayer {} wins!", winner);

            wait_for_input();
            break;
        }

        if board.iter().all(|&x| x != ' ') {
            print_board(&board);
            println!("\nIt's a draw!");

            wait_for_input();
            break;
        }

        current_player = if current_player == PLAYER1 { PLAYER2 } else { PLAYER1 };
    }
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

fn print_board(board: &Vec<char>) {
    clear_console();

    println!(" {} {} {} {} {}", format_cell(board[0], 0), color_string("|", ANSI_COLOR_YELLOW), format_cell(board[1], 1), color_string("|", ANSI_COLOR_YELLOW), format_cell(board[2], 2));
    println!("{}", color_string("---|---|---", ANSI_COLOR_YELLOW));
    println!(" {} {} {} {} {}", format_cell(board[3], 3), color_string("|", ANSI_COLOR_YELLOW), format_cell(board[4], 4), color_string("|", ANSI_COLOR_YELLOW), format_cell(board[5], 5));
    println!("{}", color_string("---|---|---", ANSI_COLOR_YELLOW));
    println!(" {} {} {} {} {}", format_cell(board[6], 6), color_string("|", ANSI_COLOR_YELLOW), format_cell(board[7], 7), color_string("|", ANSI_COLOR_YELLOW), format_cell(board[8], 8));
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn format_cell(cell: char, index: usize) -> String {
    if cell == ' ' {
        color_string(index.to_string().as_str(), ANSI_COLOR_GRAY)
    } else {
        cell.to_string()
    }
}

fn color_string(str: &str, color: &str) -> String {
    format!("{}{}{}", color, str, ANSI_COLOR_RESET)
}


fn wait_for_input() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}

