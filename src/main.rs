/*- Global allowings -*/
#![allow(dead_code)]

/*- Imports -*/
use std::io::stdin;

/*- Constants -*/
const SIZE: usize = 3;
const WINLENGTH: usize = 5;
/*- Structs, enums & unions -*/
struct Board { cells: Vec<Vec<u16>> }
// #[derive(Debug)]
// enum TileType { X,  O, NONE }


/*- Initialize -*/

fn main() {
    let mut board = Board { cells: vec![vec![0; SIZE]; SIZE] };
    println!("Enter your first move! (#, #)");

    game_loop(&mut board);
}

fn game_loop(board: &mut Board) {
    //Input and game loop
    let mut input = String::new();
    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(',').collect();
        let x: usize = input[0].trim().parse::<usize>().expect("Failed to parse input") - 1;
        let y: usize = input[1].trim().parse::<usize>().expect("Failed to parse input") - 1;
        if board.verify_move(x, y) {
            board.set_tile(x, y, 1, false);
            //run win check, origin_tile_type means you need to get the tile that it is on the board
            //win check takes current_cordinate, orgigin_cordinate and tile_type
            let tile = *board.get_tile(x, y);
            board.win_check((x, y), (x, y), tile);

            board.print_board();
        }
    }
    
}


/*- Method implementations - */
impl Board {
    pub fn get_tile(&mut self, x: usize, y: usize) -> &mut u16 {
        match self.cells.get_mut(x) {
            Some(row) => match row.get_mut(y) {
                Some(cell) => cell,
                None => panic!("Invalid Y coordinate"),
            },
            None => panic!("Invalid X coordinate"),
        }
    }
    pub fn verify_move(&mut self, x: usize, y: usize) -> bool {
        //make sure the move is valid
        //invalid is: out of bounds, or already taken
        if x >= SIZE || y >= SIZE {
            println!("Invalid move, out of bounds (Try again)");
            return false;
        }
        else if self.get_tile(x, y) != &0 {
            println!("Invalid move, already taken (Try again)");
            return false;
        } else {
            return true;
        }
    }

    pub fn win_check(&mut self, current_tile_coordinate: (usize, usize), origin_tile_coordinate: (usize, usize), origin_tile_type: u16) {
        // println!("Data\n CTC: {:?}\nOTC: {:?}\nOTT: {:?}", current_tile_coordinate, origin_tile_coordinate, origin_tile_type);
        //win algorithm
        
    }

    //exception (override) doesnt matter what is there
    pub fn set_tile(&mut self, x: usize, y: usize, value: u16, r#override: bool) {
        match self.cells.get_mut(x) {
            Some(row) => match row.get_mut(y) {
                Some(cell) => {
                    if *cell == 0 || r#override == true {
                        *cell = value;
                    }
                    else {
                        panic!("Tile is already occupied");
                    }
                },
                None => panic!("Invalid Y coordinate"),
            },
            None => panic!("Invalid X coordinate"),
        }
    }
    pub fn print_board(&mut self) {
        for (row_i, row) in &mut self.cells.iter().enumerate() {
            if row_i > 0 {
                println!("{}---","---+".repeat(SIZE-1));
            }
            for (cell_i, cell) in row.iter().enumerate() {
                if cell_i > 0 { print!(" |"); }
                match cell {
                    0 => print!("  "),
                    1 => print!(" X"),
                    2 => print!(" O"),
                    _ => panic!("Invalid value in cell"),
                }
            }
            println!();
        }
    }
}