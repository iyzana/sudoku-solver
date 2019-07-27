use itertools::Itertools;
use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Sudoku(pub Vec<u8>);

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

impl fmt::Display for Sudoku {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(fmt)?;
        writeln!(fmt, "   0 1 2   3 4 5   6 7 8 ")?;
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
        writeln!(fmt)
    }
}

impl Sudoku {
    pub fn row(&self, index: usize) -> Vec<u8> {
        self[index].to_vec()
    }

    pub fn col(&self, index: usize) -> Vec<u8> {
        (0..9).map(|row| self[row][index]).collect()
    }

    pub fn block(&self, block_x: usize, block_y: usize) -> Vec<u8> {
        (0..3)
            .cartesian_product(0..3)
            .map(|(cell_x, cell_y)| block_x * 3 + block_y * 27 + cell_x + cell_y * 9)
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

impl<'a> ops::Index<usize> for Sudoku {
    type Output = [u8];

    fn index(&self, index: usize) -> &[u8] {
        &self.0[(index * 9)..(index * 9 + 9)]
    }
}

impl ops::IndexMut<usize> for Sudoku {
    fn index_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.0[(index * 9)..(index * 9 + 9)]
    }
}
