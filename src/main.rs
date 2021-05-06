use std::{thread, time};
use rand::Rng;

struct Grid {
    grid: Vec<Vec<char>>,
    size: usize
}


const OPTIONS: [[isize; 2]; 8] = [  //all the combinations around a cell that should be checked
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1]
];

// Currently the DEAD AND ALIVE characters MUST be different
const ALIVE: char = '\u{2B1C}';
const DEAD: char = '\u{2B1B}';


impl Grid {
    fn new(x: usize) -> Self {
        Grid {
            grid: vec![vec![DEAD; x]; x], //2D vector that gets filled with the DEAD characters
            size: x
        }
    }

    fn draw(&self) {
        for row in self.grid.iter() {
            print!("\n"); //new line after every row
            for char in row.iter() {
                    if char == &ALIVE { // This is the reason why the dead and alive chars must be unique
                        print!("{}", char);
                    } else {
                        print!("{}", char);
                    }
                    
            }
        }
    }

    fn set(&mut self, row: usize, column: usize) { // Uses get_mut instead of accessing vec[0][x][y] directly because doing so can be out of bounds
        let xpos = self.grid.get_mut(row);

        match xpos { 
            Some(pos) => match pos.get_mut(column){
                Some(x) => *x = ALIVE,
                None => ()
            },
            None => ()
        }
    }

    /* Try to get the requested row and column and match the option received. If None just assume it's a dead cells. 
    This is where screen wrapping would most likely be implemented. */
    fn get(&self, row: usize, column: usize) -> char {
        let s= self.grid.get(row);

        let s = match s {
            Some(r) => match r.get(column) {
                Some(f) => f,
                None => &DEAD
            },
            None => &DEAD
        };

        *s
    }

    fn get_neighbours(&self, row: usize, column: usize) -> u8 {
        
        let mut neigbours: u8 = 0;

        for i in 0..OPTIONS.len() {
            let x = OPTIONS[i][0];
            let y = OPTIONS[i][1];

            let x2 = (row as isize - x) as usize;
            let y2 = (column as isize - y) as usize;

            let chars = self.get(x2, y2);
            if chars == ALIVE {
                neigbours += 1;
            }
        }
        neigbours
    }

    fn lives(&self, x: usize, y: usize) -> bool {
        let chara = self.get(x, y);

        if chara == ALIVE {
            if self.get_neighbours(x, y) == 2 || self.get_neighbours(x, y) == 3 {
                true
            } else {
                false
            }
            
        } else {
            if self.get_neighbours(x, y) == 3 {
                true
            } else {
                false
            }
        }
    }

    /* loops over all cells and uses rand lib to generate a number 
    in a range and depending on the number sets the cell to dead or alive */
    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for i in self.grid.iter_mut() {
            for j in i.iter_mut() {
                if rng.gen_range(0..11) > 5 {
                    *j = ALIVE;
                }
            }
        }
    }

    fn randomize_custom(&mut self, seed: u32) {
        let mut iterator = 1;

        for i in self.grid.iter_mut() {
            for j in i.iter_mut() {
                if ((2355 * (seed-iterator) + 1122) % 9925) / 1000 >= 5 {
                    *j = ALIVE;
                }
                iterator += 1;
            }
        }

    }

    //Combines all above functions in order to provide an easy way to run the game
    fn run(&mut self, frame_time: f32) {
        loop {
            let mut grid2: Grid = Grid::new(self.size); //creates a new grid of the same size as the previous
            self.draw(); // draws the current grid before it's replaced by the new one
            
            let mut row = 0; // Track row currently being checked - reset counter in next loop
    
            for i in &self.grid {
                if row < self.size { //make sure we don't go out of bounds
                    row += 1;
                }
                let mut column = 0; // Track column - reset on next row
                for _char in i.iter() {
                    if self.lives(row, column) == true { //Check if it stays alive
                        grid2.set(row, column); //if yes, set it to alive in the new grid
                    }
    
                    column += 1;
                }
            }
    
            self.grid = grid2.grid; //set primary grid equal to the updated grid
            thread::sleep(time::Duration::from_secs_f32(frame_time)); // frame time
            print!("{}[2J", 27 as char); // Clear the terminal just before the next grid is drawn
        }
    }


}

fn main() {
    let mut grid: Grid = Grid::new(30); //make a new grid
    grid.randomize(); // randomize it

    //uncomment this for a glider gun
    /* grid.set(2,2);
    grid.set(2,3);
    grid.set(2,4);
    grid.set(1,4);
    grid.set(0,3); */


    grid.run(0.25); //float

} 
