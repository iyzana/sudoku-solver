mod sudoku;
use itertools::Itertools;
use std::io;
use std::io::prelude::*;
use sudoku::Sudoku;

fn main() {
    let sudoku = read_sudoku();
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

fn solve(sudoku: &Sudoku) -> Option<Sudoku> {
    let mut sudoku = sudoku.clone();
    while fill_cells(&mut sudoku)
        || complete_elements(&mut sudoku, cells_in_col, "column")
        || complete_elements(&mut sudoku, cells_in_row, "row")
        || complete_elements(&mut sudoku, cells_in_block, "block")
    {}

    println!("{}", sudoku);

    if let Some((col, row)) = find_unfilled_cell(&sudoku) {
        let available_nums = available_nums(col, row, &sudoku);
        println!("brute forcing unfilled cell {},{}", col, row);
        println!("possible values: {:?}", available_nums);
        for num in available_nums {
            println!("trying {} at {},{}", num, col, row);
            sudoku[row][col] = num;
            if let Some(sudoku) = solve(&sudoku) {
                return Some(sudoku);
            }
            sudoku[row][col] = 0;
            println!("{}", sudoku);
        }
        println!("no valid value for cell {},{}, backtracking", col, row);
        return None;
    } else {
        println!("solved");
        return Some(sudoku);
    }
}

fn find_unfilled_cell(sudoku: &Sudoku) -> Option<(usize, usize)> {
    (0..9)
        .cartesian_product(0..9)
        .find(|&(col, row)| sudoku[row][col] == 0)
}

// find cells that only accept one number

fn fill_cells(sudoku: &mut Sudoku) -> bool {
    let unfilled_cells = (0..9)
        .cartesian_product(0..9)
        .filter(|(col, row)| sudoku[*row][*col] == 0)
        .collect::<Vec<_>>();
    unfilled_cells
        .iter()
        .any(|&(col, row)| fill_cell(col, row, sudoku))
}

// if only one number is allowed in the cell write it
fn fill_cell(col: usize, row: usize, sudoku: &mut Sudoku) -> bool {
    match available_nums(col, row, sudoku).as_slice() {
        [num] => {
            sudoku[row][col] = *num;
            println!("cell {},{} only accepts {}", col, row, num);
            true
        }
        _ => false,
    }
}

fn available_nums(col: usize, row: usize, sudoku: &Sudoku) -> Vec<u8> {
    let row_nums = sudoku.row(row);
    let col_nums = sudoku.col(col);
    let block_nums = sudoku.block(col / 3, row / 3);
    (1..=9)
        .filter(|num| !row_nums.contains(num))
        .filter(|num| !col_nums.contains(num))
        .filter(|num| !block_nums.contains(num))
        .collect()
}

// find numbers that only fit in one cell per element
// an element is a row, column or block

fn complete_elements(
    sudoku: &mut Sudoku,
    acceptable_cells: impl Fn(usize, u8, &Sudoku) -> Vec<(usize, usize)>,
    kind: &str,
) -> bool {
    (0..9).any(|element| complete_element(element, sudoku, &acceptable_cells, kind))
}

fn complete_element(
    element: usize,
    sudoku: &mut Sudoku,
    acceptable_cells: impl Fn(usize, u8, &Sudoku) -> Vec<(usize, usize)>,
    kind: &str,
) -> bool {
    (1..=9).any(
        |num| match acceptable_cells(element, num, sudoku).as_slice() {
            [(col, row)] => {
                sudoku[*row][*col] = num;
                println!("only cell {},{} accepts {} in its {}", col, row, num, kind);
                true
            }
            _ => false,
        },
    )
}

// find cells in the row that accept the number
fn cells_in_row(row: usize, num: u8, sudoku: &Sudoku) -> Vec<(usize, usize)> {
    if sudoku.row(row).contains(&num) {
        return vec![];
    }

    (0..9)
        .filter(|col| sudoku[row][*col] == 0)
        .filter(|col| !sudoku.col(*col).contains(&num))
        .filter(|col| !sudoku.block(col / 3, row / 3).contains(&num))
        .map(|col| (col, row))
        .collect()
}

// find cells in the column that accept the number
fn cells_in_col(col: usize, num: u8, sudoku: &Sudoku) -> Vec<(usize, usize)> {
    if sudoku.col(col).contains(&num) {
        return vec![];
    }

    (0..9)
        .filter(|row| sudoku[*row][col] == 0)
        .filter(|row| !sudoku.row(*row).contains(&num))
        .filter(|row| !sudoku.block(col / 3, row / 3).contains(&num))
        .map(|row| (col, row))
        .collect()
}

// find cells in the block that accept the number
fn cells_in_block(block: usize, num: u8, sudoku: &Sudoku) -> Vec<(usize, usize)> {
    let block_col = block % 3;
    let block_row = block / 3;

    if sudoku.block(block_col, block_row).contains(&num) {
        return vec![];
    }

    (0..3)
        .cartesian_product(0..3)
        .map(|(cell_col, cell_row)| (block_col * 3 + cell_col, block_row * 3 + cell_row))
        .filter(|(col, row)| sudoku[*row][*col] == 0)
        .filter(|(_, row)| !sudoku.row(*row).contains(&num))
        .filter(|(col, _)| !sudoku.col(*col).contains(&num))
        .collect()
}
