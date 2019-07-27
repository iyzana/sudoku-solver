use itertools::Itertools;
use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Sudoku(pub Vec<u8>);

impl Sudoku {
    pub fn row(&self, row: usize) -> Vec<u8> {
        self[row].to_vec()
    }

    pub fn col(&self, col: usize) -> Vec<u8> {
        (0..9).map(|row| self[row][col]).collect()
    }

    pub fn block(&self, block_col: usize, block_row: usize) -> Vec<u8> {
        (0..3)
            .cartesian_product(0..3)
            .map(|(cell_col, cell_row)| block_col * 3 + block_row * 27 + cell_col + cell_row * 9)
            .map(|index| self.0[index])
            .collect()
    }

    pub fn is_valid(&self) -> bool {
        (0..9).all(|element| {
            Self::has_no_duplicates(self.row(element))
                && Self::has_no_duplicates(self.col(element))
                && Self::has_no_duplicates(self.block(element % 3, element / 3))
        })
    }

    fn has_no_duplicates(nums: Vec<u8>) -> bool {
        let mut nums = nums.into_iter().filter(|&num| num != 0).collect::<Vec<_>>();
        let len = nums.len();
        nums.sort();
        nums.dedup();
        len == nums.len()
    }
}

impl From<&str> for Sudoku {
    fn from(sudoku: &str) -> Self {
        let nums = sudoku
            .lines()
            .flat_map(str::chars)
            .map(|c| c.to_string().parse().unwrap_or(0))
            .collect::<Vec<_>>();

        let sudoku = Self(nums);

        if sudoku.0.len() != 81 {
            panic!("sudoku is incomplete");
        }
        if !sudoku.is_valid() {
            panic!("sudoku is not valid");
        }

        sudoku
    }
}

impl<'a> ops::Index<usize> for Sudoku {
    type Output = [u8];

    fn index(&self, row: usize) -> &[u8] {
        &self.0[(row * 9)..(row * 9 + 9)]
    }
}

impl ops::IndexMut<usize> for Sudoku {
    fn index_mut(&mut self, row: usize) -> &mut [u8] {
        &mut self.0[(row * 9)..(row * 9 + 9)]
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(fmt)?;
        writeln!(fmt, "   a b c   d e f   g h i ")?;
        writeln!(fmt)?;
        for i in 0..81 {
            if i % 9 == 0 {
                write!(fmt, "{}  ", i / 9)?;
            }
            match self.0[i] {
                0 => write!(fmt, "  ")?,
                n => write!(fmt, "{} ", n)?,
            }
            if i % 3 == 2 && i % 9 != 8 {
                write!(fmt, "| ")?;
            }
            if i % 9 == 8 {
                writeln!(fmt)?;
            }
            if i % 27 == 26 && i % 81 != 80 {
                writeln!(fmt, "  -----------------------")?;
            }
        }
        Ok(())
    }
}

pub fn fmt_cell(col: usize, row: usize) -> String {
    format!("{}{}", (col as u8 + b'a') as char, row)
}
