# Conway's Game of Life
## A very simple implementation of Conway's Game of life in the terminal using Rust.
---

It can most certainly be improved by:
 * Making a struct for cells and then an enum to track their state or,
 
 * by implementing opengl rendering or using a graphics framework for more advanced visual,

 * separating code into modules,

 * a million other ways.
 
 Even though how it is now is very crude and full of beginner mistakes, this was a project I started not even half way into the Rust book and with no experience with compiled languages.

 ## Built using:
---
 * Std lib
 * Random crate (could be replaced with custom implementation, used once)


## Usage:
 ---
* Clone and `cargo run`
* Change grid size if desired at `line 160`:

```rust
let mut grid: Grid = Grid::new(30);
```
* Change the characters used for the dead and alive on `lines 22 and 23`

## Demo:
![alt text](https://i.imgur.com/vpdtf9a.gif "Demo")


## Known bugs:
---
* Using the same chars for both dead and alive state doesn't work due to the current implementation.