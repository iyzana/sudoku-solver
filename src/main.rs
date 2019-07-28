mod solver;
mod sudoku;

use solver::solve;
use std::io::{self, prelude::*};
use sudoku::Sudoku;

fn main() {
    let sudoku = read_sudoku();
    // let sudoku = Sudoku::from(include_str!("./data/sudoku-evil-0.txt"));
    println!("{}", sudoku);

    if let Some(solution) = solve(&sudoku) {
        println!("{}", solution);
    } else {
        println!("no solution");
    }
}

fn read_sudoku() -> Sudoku {
    println!("input sudoku (9 lines, 9 characters each):");
    let input = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| format!("{:0<9}", line))
        .flat_map(|s| s.chars().collect::<Vec<_>>())
        .take(81)
        .collect::<String>();

    Sudoku::from(input.as_str())
}
