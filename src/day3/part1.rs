use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::path::Path;

struct Cell {
    value: char,
    visited: bool,
}

pub fn run() -> Result<i32, Error> {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    if let Ok(lines) = read_lines("src/day3/input.txt") {
        for line in lines {
            grid.push(line_to_cell_vec(line?.as_str()))
        }
    } else {
        panic!("File could not be read.");
    }
    Ok(calculate_sum(&mut grid))
}

fn calculate_sum(grid: &mut Vec<Vec<Cell>>) -> i32 {

    assert!(grid.len() > 0);
    let col_len = grid[0].len();
    let mut sum: i32 = 0;

    for row_idx in 0..grid.len() {
        for col_idx in 0..col_len {
            if is_symbol(&grid[row_idx][col_idx].value) {

                // North
                if is_unvisited_digit(&grid, row_idx as i16 - 1, col_idx as i16) {
                    sum += try_visiting_number(&mut grid[row_idx.saturating_sub(1)], col_idx);
                }

                // Northeast
                if is_unvisited_digit(&grid, row_idx as i16 - 1, col_idx as i16 + 1) {
                    sum += try_visiting_number(&mut grid[row_idx.saturating_sub(1)], col_idx.saturating_add(1));
                }

                // East
                if is_unvisited_digit(&grid, row_idx as i16, col_idx as i16 + 1) {
                    sum += try_visiting_number(&mut grid[row_idx], col_idx.saturating_add(1));
                }

                // Southeast
                if is_unvisited_digit(&grid, row_idx as i16 + 1, col_idx as i16 + 1) {
                    sum += try_visiting_number(&mut grid[row_idx.saturating_add(1)], col_idx.saturating_add(1));
                }

                // South
                if is_unvisited_digit(&grid, row_idx as i16 + 1, col_idx as i16) {
                    sum += try_visiting_number(&mut grid[row_idx.saturating_add(1)], col_idx);
                }

                // Southwest
                if is_unvisited_digit(&grid, row_idx as i16 + 1, col_idx as i16 - 1) {
                    sum += try_visiting_number(&mut grid[row_idx.saturating_add(1)], col_idx.saturating_sub(1));
                }

                // West
                if is_unvisited_digit(&grid, row_idx as i16, col_idx as i16 - 1) {
                    sum += try_visiting_number(&mut grid[row_idx], col_idx.saturating_sub(1));
                }

                // Northwest
                if is_unvisited_digit(&grid, row_idx as i16 - 1, col_idx as i16 - 1) {
                    sum += try_visiting_number(&mut grid[row_idx.saturating_sub(1)], col_idx.saturating_sub(1));
                }
            }
        }
    }

    sum
}

fn is_symbol(c: &char) -> bool {
    !(c.is_digit(10) || *c == '.')
}

fn is_unvisited_digit(grid: &Vec<Vec<Cell>>, row_idx: i16, col_idx: i16) -> bool {
    row_idx >= 0 && row_idx < grid.len() as i16
        && col_idx >= 0 && col_idx < grid[row_idx as usize].len() as i16
        && !grid[row_idx as usize][col_idx as usize].visited
        && grid[row_idx as usize][col_idx as usize].value.is_digit(10)
}

fn try_visiting_number(row: &mut Vec<Cell>, col_start_idx: usize) -> i32 {
    if col_start_idx >= row.len() || row[col_start_idx].visited || !row[col_start_idx].value.is_digit(10) {
        return 0;
    }

    let row_len = row.len() as i16;
    let mut num_vec: VecDeque<char> = VecDeque::new();

    num_vec.push_back(row[col_start_idx].value.clone());
    row[col_start_idx].visited = true;

    let mut col_idx = col_start_idx as i16 + 1;
    while col_idx < row_len && row[col_idx as usize].value.is_digit(10) {
        row[col_idx as usize].visited = true;
        num_vec.push_back(row[col_idx as usize].value.clone());
        col_idx += 1;
    }

    col_idx = col_start_idx as i16 - 1;
    while col_idx >= 0 && row[col_idx as usize].value.is_digit(10) {
        row[col_idx as usize].visited = true;
        num_vec.push_front(row[col_idx as usize].value.clone());
        col_idx -= 1;
    }

    String::from_iter(num_vec).parse::<i32>().unwrap()
}

fn line_to_cell_vec(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| Cell { value: c, visited: false })
        .collect()
}

fn read_lines<A>(filename: A) -> io::Result<io::Lines<io::BufReader<File>>> where A: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unvisited_test_row(test_row: &mut Vec<Cell>) -> &mut Vec<Cell> {
        test_row.iter_mut().for_each(|col| col.visited = false);
        test_row
    }

    fn generate_test_lines() -> Vec<Vec<Cell>> {
        let mut lines: Vec<Vec<Cell>> = Vec::new();
        lines.push(line_to_cell_vec("467..114.."));
        lines.push(line_to_cell_vec("...*......"));
        lines.push(line_to_cell_vec("..35..633."));
        lines.push(line_to_cell_vec("......#..."));
        lines.push(line_to_cell_vec("617*......"));
        lines.push(line_to_cell_vec(".....+.58."));
        lines.push(line_to_cell_vec("..592....."));
        lines.push(line_to_cell_vec("......755."));
        lines.push(line_to_cell_vec("...$.*...."));
        lines.push(line_to_cell_vec(".664.598.."));
        lines
    }

    #[test]
    fn test_sum() {
        let mut grid = generate_test_lines();
        assert_eq!(4361, calculate_sum(&mut grid));
    }

    #[test]
    fn test_discover_number() {
        let mut test_row = line_to_cell_vec("123..456..789");

        assert_eq!(123, try_visiting_number(unvisited_test_row(&mut test_row), 0));
        assert_eq!(123, try_visiting_number(unvisited_test_row(&mut test_row), 1));
        assert_eq!(123, try_visiting_number(unvisited_test_row(&mut test_row), 2));

        assert_eq!(456, try_visiting_number(unvisited_test_row(&mut test_row), 5));
        assert_eq!(456, try_visiting_number(unvisited_test_row(&mut test_row), 6));
        assert_eq!(456, try_visiting_number(unvisited_test_row(&mut test_row), 7));

        assert_eq!(789, try_visiting_number(unvisited_test_row(&mut test_row), 10));
        assert_eq!(789, try_visiting_number(unvisited_test_row(&mut test_row), 11));
        assert_eq!(789, try_visiting_number(unvisited_test_row(&mut test_row), 12));

        assert_eq!(0, try_visiting_number(unvisited_test_row(&mut test_row), 13));
        assert_eq!(0, try_visiting_number(unvisited_test_row(&mut test_row), 3));
    }

}
