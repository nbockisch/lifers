# Lifers

## About
[Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implemented in [Rust](https://www.rust-lang.org/) with [Ncurses](https://invisible-island.net/ncurses/). Written for fun to
practice Rust and learn about the Game of Life. It currently supports pattern 
files in the [Plaintext](https://conwaylife.com/wiki/Plaintext) (.cells) format.

## Dependencies
* [Cargo](https://doc.rust-lang.org/cargo/) - For building
* Ncurses

## Building
The project has currently been tested on Linux, other platforms are untested.

1. `git clone` the repository
2. `cd` to it
3. `cargo build --release`
4. The binary will be at `target/release/lifers`

## Usage
```
lifers 0.1.0
Nathan Bockisch <nbockisch@protonmail.com>
Implementation of Conway's Game of Life in Rust and Ncurses

USAGE:
    lifers [OPTIONS] <PATTERN_PATH>

ARGS:
    <PATTERN_PATH>    Path to the pattern file

OPTIONS:
    -a, --around             Wrap cells around edges (default is no)
    -h, --height <HEIGHT>    Height of the map (default is height of terminal)
        --help               Print help information
    -V, --version            Print version information
    -w, --width <WIDTH>      Width of the map (default is width of terminal)
```

## Issues/Further Improvements
* I may add support for other pattern file formats
* There is currently a screen flashing graphical glitch that happens with complex patterns using the wrap around option
