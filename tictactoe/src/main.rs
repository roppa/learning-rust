/**
 * Tic Tac Toe or Naughts and crosses
 * 
 * Two players take turns with adding their moniker on a 3x3 grid with the aim of having a winning swequence:
 * 
 * Winning sequences are:
 * - first line horizontal
 * - second line horizontal
 * - third line horizontal
 * - first line vertical
 * - second line vertical
 * - third line vertical
 * - diagonal l - r
 * - diagonal r - l
 **/

use tictactoe::check_results;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lookup = ["a1",  "b1",  "c1", "a2",  "b2",  "c2", "a3",  "b3",  "c3"];
    let players = ["x", "o"];
    let mut current_player = 0;
    let mut matrix: [String; 9] = Default::default();

    println!("Tic-tac-toe");
    println!("Ready player {}? Enter target square:", players[current_player].to_string());
    println!("|----|----|----|");
    println!("| a1 | b1 | c1 |");
    println!("|____|____|____|");
    println!("| a2 | b2 | c3 |");
    println!("|____|____|____|");
    println!("| a3 | b3 | c3 |");
    println!("|____|____|____|");

    for line in stdin.lock().lines() {
        let input = line.unwrap();
        let index = lookup.iter().position(|&r| r == input);
        if index.is_some() {
            if matrix[index.unwrap()].to_string() != "" {
                println!("Already taken");
                continue;
            }
            matrix[index.unwrap()] = players[current_player].to_string();
            current_player = if current_player == 0 { 1 } else { 0 };
            if check_results(&matrix) {
                std::process::exit(1);
            }
            println!("Player {}, your go", players[current_player].to_string());
        } else {
           println!("Oops! Invalid square");
        }
    }
}
