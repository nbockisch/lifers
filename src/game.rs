use std::{error::Error, thread, time};

use crate::loader;

pub struct Cell {
    pub is_alive: bool,
    ypos: i32,
    xpos: i32,
    pub neighbors: u8,
}

impl Cell {
    fn draw(&self) {
        ncurses::mv(self.ypos, self.xpos);

        let c = match self.is_alive {
            true => '#' as u32,
            false => ' ' as u32,
        };

        ncurses::addch(c);
        // ncurses::addstr(format!("{}", self.neighbors).as_str());
    }
}

/// The main launching point for the game, contains the game loop
///
/// # Errors
/// This function propogates errors from the file loader, parser, and cell
/// updater functions
pub fn run_game(args: crate::Args) -> Result<(), Box<dyn Error>> {
    ncurses::initscr();
    ncurses::nodelay(ncurses::stdscr(), true);
    // ncurses::keypad(ncurses::stdscr(), true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Use custom dimensions if they exist
    let height = match args.height {
        Some(h) => h,
        None => ncurses::getmaxy(ncurses::stdscr()),
    };

    let width = match args.width {
        Some(w) => w,
        None => ncurses::getmaxx(ncurses::stdscr()),
    };

    let mut cells: Vec<Vec<Cell>> = Vec::new();

    // Initialize cells
    for ypos in 0..height {
        cells.push(Vec::new());

        for xpos in 0..width {
            let new_cell = Cell {
                is_alive: false,
                ypos,
                xpos,
                neighbors: 0,
            };

            new_cell.draw();

            cells.get_mut(ypos as usize)
                .unwrap()
                .push(new_cell);
        }
    }

    // Populate the cells with the pattern
    let mut update_indexes = match loader::load_path(args.pattern_path, &cells) {
        Ok(list) => list,
        Err(e) => {
            ncurses::endwin();
            return Err(e);
        }
    };

    // Game loop
    // Break on q or ESC
    let mut c = 0;
    while (c != 'q' as i32) && (c != 27) {
        c = ncurses::getch();

        update_at_indexes(&mut cells, &update_indexes, args.around);
        draw_cells(&cells, &update_indexes);
        update_indexes = get_updates(&cells);
        thread::sleep(time::Duration::from_millis(100));
    }

    ncurses::endwin();
    Ok(())
}

/// Update the cells at given indexes by swapping their alive status, and
/// update their neighbors accordingly
fn update_at_indexes(cells: &mut Vec<Vec<Cell>>, indexes: &Vec<(usize, usize)>, should_wrap: bool) {
    for index in indexes {
        let row = index.0;
        let col = index.1;
        let mut new_status = false;

        // Swap alive status
        if let Some(row_vec) = cells.get_mut(row) {
            if let Some(cell) = row_vec.get_mut(col) {
                cell.is_alive ^= true;
                new_status = cell.is_alive;
            } else { continue; }
        }

        // Get neighbor rows, wrapping if requested
        let row_iter: Box<dyn Iterator<Item = usize>> = match row {
            0 => {
                if should_wrap {
                    Box::new([row, row + 1, cells.len() - 1].into_iter())
                } else { 
                    Box::new(row..=(row + 1))
                }
            },
            x if x == (cells.len() - 1) => {
                if should_wrap {
                    Box::new([row - 1, row, 0].into_iter())
                } else {
                    Box::new(row..=(row + 1))
                }
            }
            _ => Box::new((row - 1)..=(row + 1)),
        };

        // Update neighbors
        for i in row_iter {
            let n_row = match cells.get_mut(i) {
                Some(n_row) => n_row,
                None => continue,
            };

            // Get neighbor columns, wrapping if requested
            let col_iter: Box<dyn Iterator<Item = usize>> = match col {
                0 => {
                    if should_wrap {
                        Box::new([col, col + 1, n_row.len() - 1].into_iter())
                    } else { 
                        Box::new(col..=(col + 1))
                    }
                },
                x if x == (n_row.len() - 1) => {
                    if should_wrap {
                        Box::new([col - 1, col, 0].into_iter())
                    } else {
                        Box::new((col - 1)..=(col + 1))
                    }
                }
                _ => Box::new((col - 1)..=(col + 1)),
            };

            for j in col_iter {
                // Don't update the neighbor count of the original cell
                if (i == row) && (j == col) { continue; }

                let mut n_cell = match n_row.get_mut(j) {
                    Some(n_cell) => n_cell,
                    None => continue,
                };

                match new_status {
                    true => n_cell.neighbors += 1,
                    false => n_cell.neighbors -= 1,
                }
            }
            if col == 0 {
                println!("");
            }
        }
    }
}

/// Draw cells that have been updated in the last iteration
fn draw_cells(cells: &Vec<Vec<Cell>>, indexes: &Vec<(usize, usize)>) {
   // ncurses::clear();
    for index in indexes {
        let row = index.0;
        let col = index.1;

        if let Some(row_vec) = cells.get(row) {
            if let Some(cell) = row_vec.get(col) {
                cell.draw();
            }
        }
    }
    ncurses::refresh();
}

/// Check neighbor counts for all cells to see which need to be updated, and
/// return the list of indexes to update
fn get_updates(cells: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut update_indexes: Vec<(usize, usize)> = Vec::new();

    for row in cells {
        for cell in row {
            match cell.is_alive {
                true => {
                    if (cell.neighbors < 2) || (cell.neighbors > 3) {
                        update_indexes.push((cell.ypos as usize, cell.xpos as usize));
                    }
                },
                false => if cell.neighbors == 3 {
                    update_indexes.push((cell.ypos as usize, cell.xpos as usize));
                },
            }
        }
    }

    update_indexes
}
