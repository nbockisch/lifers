use std::{fs::File, path::Path, error::Error, ffi::OsStr, io::{BufReader, BufRead}};

use crate::game;

/// Populates a Vec of Cell structs with information from a pattern file
///
/// # Errors
/// This function will return an error if it fails to load a file at the given
/// path, if the file extension for the path is not recognized, or if the parsing
/// function for a discovered filetype returns an error.
pub fn load_path(path: impl AsRef<Path>, cells: &Vec<Vec<game::Cell>>)
    -> Result<Vec<(usize, usize)>, Box<dyn Error>> {

    let pattern = File::open(&path)?;

    let extension = path
        .as_ref()
        .extension()
        .and_then(OsStr::to_str);

    match extension {
        Some("cells") => parse_plaintext(pattern, cells),
        _ => Err(format!("Unknown file extension: {:?}", extension))?,
    }
}

/// Parses a [plaintext](https://conwaylife.com/wiki/Plaintext) pattern file and
/// populates a 2D Vector of Cell structs with the results
///
/// #Errors
/// This function will return an error if the pattern file has a syntax error or
/// if there is an error iterating through the BufReader
fn parse_plaintext(pattern: File, cells: &Vec<Vec<game::Cell>>)
    -> Result<Vec<(usize, usize)>, Box<dyn Error>> {

    let reader = BufReader::new(&pattern);

    let board_height = cells.len();
    let board_width = match cells.get(0) {
        Some(row) => row.len(),
        None => return Err("Cell vector appears to be empty")?,
    };

    let mut update_indexes: Vec<(usize, usize)> = Vec::new();

    let mut row = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line?;

        // Check for comment or empty line
        if line.is_empty() { continue; }

        let first_char = line.replace(" ", "").chars().next();
        if let Some('!') = first_char {
            continue;
        }

        // Make sure pattern falls within dimensions of board
        let line_len = line.chars().count();
        if (row >= board_height) || (line_len > board_width) {
            return Err(format!("Pattern file exceeds the dimensions of the board: {}x{}",
            board_width, board_height))?;
        }

        // Set cells in pattern file to be alive
        for (col, symbol) in line.chars().enumerate() {
            match symbol {
                'O' => {
                    if let Some(row_vec) = cells.get(row) {
                        if let Some(_) = row_vec.get(col) {
                            update_indexes.push((row, col));
                        }
                    }
                },
                '.' | ' ' => continue,
                e => return Err(format!("Invalid character: {}", e))?,
            }
        }

        row += 1;
    }

    Ok(update_indexes)
}
