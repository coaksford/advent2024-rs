use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    // gets input into a grid of chars.
    let grid = get_input()?;

    // horizontal can be done linewise.
    let horizontal: i32 = grid.iter().map(search_horizontal).sum();

    // vertical has to be done in bands of 4 lines.
    let vertical: i32 = grid.windows(4).map(search_vertical).sum();

    // diagonal also has to be done in bands of 4 lines.
    let diagonal: i32 = grid.windows(4).map(search_diagonal).sum();

    let total = horizontal + vertical + diagonal;

    println!("horizontal = {horizontal}");
    println!("vertical = {vertical}");
    println!("diagonal = {diagonal}");
    println!("total = {total}");
    
    Ok(())
}

fn get_input() -> Result<Vec<Vec<char>>> {
    let mut input = File::open("sample-day4.txt")?;
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    let lines = text.lines();
    
    let grid = lines.map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    Ok(grid)
}

const X: Option<&char> = Some(&'X');
const M: Option<&char> = Some(&'M');
const A: Option<&char> = Some(&'A');
const S: Option<&char> = Some(&'S');

fn search_horizontal(line: &Vec<char>) -> i32 {
    let mut count = 0;
    for window in line.windows(4) {
        if (
            window.get(0) == X &&
            window.get(1) == M &&
            window.get(2) == A &&
            window.get(3) == S
        ) || (
            window.get(3) == X &&
            window.get(2) == M &&
            window.get(1) == A &&
            window.get(0) == S
        ) {
            count += 1;
        }
    }

    count
}

fn search_vertical(band: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let len = band.get(0).unwrap_or(&vec![]).len();
    for column in 0..len {        
        if ( // top to bottom
            band.get(0).and_then(|row| row.get(column)) == X &&
            band.get(1).and_then(|row| row.get(column)) == M &&
            band.get(2).and_then(|row| row.get(column)) == A &&
            band.get(3).and_then(|row| row.get(column)) == S
        ) || (
            // bottom to top
            band.get(3).and_then(|row| row.get(column)) == X &&
            band.get(2).and_then(|row| row.get(column)) == M &&
            band.get(1).and_then(|row| row.get(column)) == A &&
            band.get(0).and_then(|row| row.get(column)) == S
        ) {
            count += 1;
        }
    }

    count
}

fn search_diagonal(band: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let len = band.get(0).unwrap_or(&vec![]).len();

    for column in 0..len {        
        if  // NE-SW: first index ascending, second ascending
            band.get(0).and_then(|row| row.get(column+0)) == X &&
            band.get(1).and_then(|row| row.get(column+1)) == M &&
            band.get(2).and_then(|row| row.get(column+2)) == A &&
            band.get(3).and_then(|row| row.get(column+3)) == S
        {
            count += 1;
        }

        if  // NW-SE: first index ascending, second descending.
            band.get(0).and_then(|row| row.get(column+3)) == X &&
            band.get(1).and_then(|row| row.get(column+2)) == M &&
            band.get(2).and_then(|row| row.get(column+1)) == A &&
            band.get(3).and_then(|row| row.get(column+0)) == S
        {
            count += 1;
        }

        if  // SE-NW: first index descending, second ascending
            band.get(3).and_then(|row| row.get(column+0)) == X &&
            band.get(2).and_then(|row| row.get(column+1)) == M &&
            band.get(1).and_then(|row| row.get(column+2)) == A &&
            band.get(0).and_then(|row| row.get(column+3)) == S
        {
            count += 1;
        }

        if  // SW-NE: first index descending, second ascending
            band.get(3).and_then(|row| row.get(column+3)) == X &&
            band.get(2).and_then(|row| row.get(column+2)) == M &&
            band.get(1).and_then(|row| row.get(column+1)) == A &&
            band.get(0).and_then(|row| row.get(column+0)) == S
        {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonal() {
        let test = |grid: Vec<Vec<char>>| grid.windows(4).map(search_diagonal).sum::<i32>();

        let grid_nwse = vec![
            vec!['X', ' ', ' ', ' '],
            vec![' ', 'M', ' ', ' '],
            vec![' ', ' ', 'A', ' '],
            vec![' ', ' ', ' ', 'S'],
        ];

        let grid_nesw = vec![
            vec![' ', ' ', ' ', 'X'],
            vec![' ', ' ', 'M', ' '],
            vec![' ', 'A', ' ', ' '],
            vec!['S', ' ', ' ', ' '],
        ];

        let grid_swne = vec![
            vec![' ', ' ', ' ', 'S'],
            vec![' ', ' ', 'A', ' '],
            vec![' ', 'M', ' ', ' '],
            vec!['X', ' ', ' ', ' '],
        ];

        let grid_senw = vec![
            vec!['S', ' ', ' ', ' '],
            vec![' ', 'A', ' ', ' '],
            vec![' ', ' ', 'M', ' '],
            vec![' ', ' ', ' ', 'X'],
        ];

        let grid_overlapping = vec![
            vec!['X', ' ', ' ', 'X'],
            vec![' ', 'M', 'M', ' '],
            vec![' ', 'A', 'A', ' '],
            vec!['S', ' ', ' ', 'S'],
        ];

        assert_eq!(test(grid_nwse), 1i32);
        assert_eq!(test(grid_nesw), 1i32);
        assert_eq!(test(grid_swne), 1i32);
        assert_eq!(test(grid_senw), 1i32);
        assert_eq!(test(grid_overlapping), 2i32);

    }

    #[test]
    fn vertical() {
        let test = |grid: Vec<Vec<char>>| grid.windows(4).map(search_vertical).sum::<i32>();

        let grid = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['M', 'A', 'M', 'A'],
            vec!['A', 'M', 'A', 'M'],
            vec!['S', 'M', 'A', 'X'],
        ];

        assert_eq!(test(grid), 2i32);

    }

    #[test]
    fn horizontal() {
        let test = |grid: Vec<Vec<char>>| grid.iter().map(search_horizontal).sum::<i32>();

        let grid = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['S', 'A', 'M', 'X'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];

        assert_eq!(test(grid), 4i32);
    }
}
