use crate::sudoku::{fmt_cell, Sudoku};
use itertools::Itertools;

pub fn solve(sudoku: &Sudoku) -> Option<Sudoku> {
    let mut sudoku = sudoku.clone();
    apply_rules(&mut sudoku);
    println!("{}", sudoku);
    brute_force(sudoku)
}

fn apply_rules(sudoku: &mut Sudoku) {
    // simple rules
    while fill_cells(sudoku)
        || complete_elements(sudoku, cells_in_col, "column")
        || complete_elements(sudoku, cells_in_row, "row")
        || complete_elements(sudoku, cells_in_block, "block")
    {}
}

fn brute_force(mut sudoku: Sudoku) -> Option<Sudoku> {
    // brute force
    if let Some((col, row, available_nums)) = find_unfilled_cell(&sudoku) {
        println!(
            "bruteforcing {:?} for cell {}",
            available_nums,
            fmt_cell(col, row)
        );
        for num in available_nums {
            println!("assuming {} for cell {}", num, fmt_cell(col, row));
            sudoku[row][col] = num;
            if let Some(sudoku) = solve(&sudoku) {
                return Some(sudoku);
            }
            println!("{}", sudoku);
        }
        println!(
            "no valid value for cell {}, backtracking",
            fmt_cell(col, row)
        );
        None
    } else {
        println!("solved");
        Some(sudoku)
    }
}

fn find_unfilled_cell(sudoku: &Sudoku) -> Option<(usize, usize, Vec<u8>)> {
    (0..9)
        .cartesian_product(0..9)
        .filter(|&(col, row)| sudoku[row][col] == 0)
        .map(|(col, row)| (col, row, available_nums(col, row, sudoku)))
        .min_by_key(|(_, _, available_nums)| available_nums.len())
}

// find cells that only accept one number

fn fill_cells(sudoku: &mut Sudoku) -> bool {
    (0..9).cartesian_product(0..9).any(|(col, row)| {
        sudoku[row][col] == 0 && fill_cell(col, row, sudoku)
    })
}

// if only one number is allowed in the cell write it
fn fill_cell(col: usize, row: usize, sudoku: &mut Sudoku) -> bool {
    match available_nums(col, row, sudoku).as_slice() {
        [num] => {
            sudoku[row][col] = *num;
            println!("cell {} only accepts {}", fmt_cell(col, row), num);
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
                println!(
                    "only cell {} accepts {} in its {}",
                    fmt_cell(*col, *row),
                    num,
                    kind
                );
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
