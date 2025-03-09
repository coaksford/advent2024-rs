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
        if (window.get(0) == X &&
           window.get(1) == M &&
           window.get(2) == A &&
           window.get(3) == S)
        ||
           (window.get(3) == X &&
           window.get(2) == M &&
           window.get(1) == A &&
           window.get(0) == S)
        {
            count += 1;
        }
    }

    count
}

fn search_vertical(band: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let len = band.get(0).unwrap_or(&vec![]).len();
    for column in 0..len {        
        if (band.get(0).and_then(|row| row.get(column)) == X &&
            band.get(1).and_then(|row| row.get(column)) == M &&
            band.get(2).and_then(|row| row.get(column)) == A &&
            band.get(3).and_then(|row| row.get(column)) == S)
            || (band.get(0).and_then(|row| row.get(column)) == X &&
            band.get(1).and_then(|row| row.get(column)) == M &&
            band.get(2).and_then(|row| row.get(column)) == A &&
            band.get(3).and_then(|row| row.get(column)) == S)
        {
            count += 1;
        }
    }

    count
}

fn search_diagonal(band: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let len = band.get(0).unwrap_or(&vec![]).len();

    for column in 0..(len-3) {        
        if (band.get(0).and_then(|row| row.get(column)) == X &&
            band.get(1).and_then(|row| row.get(column+1)) == M &&
            band.get(2).and_then(|row| row.get(column+2)) == A &&
            band.get(3).and_then(|row| row.get(column+3)) == S)
            || (band.get(0).and_then(|row| row.get(column)) == X &&
            band.get(1).and_then(|row| row.get(column+1)) == M &&
            band.get(2).and_then(|row| row.get(column+2)) == A &&
            band.get(3).and_then(|row| row.get(column+3)) == S)
            || (band.get(3).and_then(|row| row.get(column)) == X &&
            band.get(2).and_then(|row| row.get(column+1)) == M &&
            band.get(1).and_then(|row| row.get(column+2)) == A &&
            band.get(0).and_then(|row| row.get(column+3)) == S)
            || (band.get(3).and_then(|row| row.get(column)) == X &&
            band.get(2).and_then(|row| row.get(column+1)) == M &&
            band.get(1).and_then(|row| row.get(column+2)) == A &&
            band.get(0).and_then(|row| row.get(column+3)) == S)
        {
            count += 1;
        }
    }

    count
}


