/*- Global allowings -*/
#![allow(dead_code, unused_variables, non_snake_case, unused_mut)]

/*- Imports -*/
use std::io::stdin;
use std::{env, vec};

/*- Constants -*/
const SIZE: usize = 3;
const WINLENGTH: usize = 3;
/*- Structs, enums & unions -*/
struct Board { cells: Vec<Vec<u16>> }
// #[derive(Debug)]
// enum TileType { X,  O, NONE }


/*- Initialize -*/

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let mut board = Board { cells: vec![vec![0; SIZE]; SIZE] };
    println!("Enter your first move! (#, #)");

    game_loop(&mut board);
}

fn game_loop(board: &mut Board) {
    //Input and game loop
    let mut input = String::new();
    let mut turn = 0;
    let mut player = 0;
    let mut winner = 0;
    loop {
        
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(',').collect();
        let x: usize = input[0].trim().parse::<usize>().expect("Failed to parse input") - 1;
        let y: usize = input[1].trim().parse::<usize>().expect("Failed to parse input") - 1;

        if turn >= (SIZE*SIZE) as u16 && winner == 0 { println!("Board full, tie!"); break }

        if !board.oob(x, y) && board.empty_tile(x, y) {
            turn += 1;
            // player = (turn+1)%2+1;
            player = 1;
            board.set_tile(x, y, player, false);
            //run win check, origin_tile_type means you need to get the tile that it is on the board
            //win check takes current_cordinate, orgigin_cordinate and tile_type
            // let tile = *board.get_tile(x, y);
            board.print_board();
            winner = board.win_check(player);
        } 

        while winner != 0 {
            println!("Player {} wins!", winner);
            break;
        }
    }

}


fn add(a: usize, b: i32) -> usize {
    if b.is_negative() {
        a - b.wrapping_abs() as u32 as usize
    } else {
        a + b as usize
    }
}


/*- Method implementations - */
impl Board {
    pub fn get_tile(&mut self, x: usize, y: usize) -> &mut u16 {
        match self.cells.get_mut(y) {
            Some(row) => match row.get_mut(x) {
                Some(cell) => cell,
                None => panic!("Invalid X coordinate"),
            },
            None => panic!("Invalid Y coordinate"),
        }
    }
    pub fn empty_tile(&mut self, x: usize, y: usize) -> bool {
        if self.get_tile(x, y) != &0 {
            println!("Invalid move, already taken (Try again)");
            return false;
        } else {
            return true;
        }
    }

    pub fn ass(
        &mut self,
        current_tile_coordinate: (usize, usize),
        origin_tile_coordinate: (usize, usize),
        origin_tile_type: u16,
        mut counter: usize
    ) {
        // println!("Data\n CTC: {:?}\nOTC: {:?}\nOTT: {:?}", current_tile_coordinate, origin_tile_coordinate, origin_tile_type);
        //win algorithm


            /*
            Vi kommer att titta längst Y axeln (också den andra indexet i tuplen) om den är 0 eller SIZE
            för att veta om vi ska skippa titta åt ena hållet. Om det inte är 0 eller SIZE
            börjar vi med att titta uppåt genom att recusivly calla samma function igen men
            med en annan current_tile_coordinate (y+1), ha kvar orgin tile coordinate och tile type.
            Kolla om denna nya tile är lika med orgin tile type. Om den är det, addera 1 till en counter.
            Fortsätt tills att det har kommit till en tile som inte är lika med orgin tile type.
            När det har kommit till en tile som inte är lika med orgin tile type, calla samma function som i början
            men istället för y+1 på current_tile_coordinate så ska det vara y-1. Recusa detta tills att
            counter är lika med WINLENGTH eller att det har kommit till en tile som inte är lika med orgin tile type.

            !Mellan varje steg så måste vi kolla om counter är lika med WINLENGTH. Om den är det så har vi vunnit.
            */
        //Titta om vi inte är på den fösta tilen eller den sista

        //=----------------------=
        //These are the positions to check
        let position_coords: &[[(i32,i32);2]] = &[
            [(1,0), (-1,0)], // right and left
            [(0,1), (0,-1)] //up and down
            ];
        //The tuples in the same array need to be added togheter to get the total count. This is because we need to look both in the negativt and the positve direction for each axis, add them togheter. Before we get the tile we need to check if the tile is in bounds, if it is not we need to skip it. (oob)
        //when recursing we need to save the direction we are going in, so we can skip the other direction when we are recursing and make sure we do not go back and forth. (skip)
        //The counter is the total count of the tiles that are the same as the origin tile. If the counter is equal to WINLENGTH then we have won.
        

        
    }

    pub fn win_check(&mut self, player: u16) -> u16 { //this method is pretty scuffed and not very efficent.
        //begin checking every column, use println for debugging
        let mut counter = 0;
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.get_tile(x, y) == &player {
                    counter += 1;
                    if counter == WINLENGTH {
                        return player;
                    }
                } else {
                    counter = 0;
                }
            }
        }
        counter = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if self.get_tile(x, y) == &player {
                    counter += 1;
                    if counter == WINLENGTH {
                        return player;
                    }
                } else {
                    counter = 0;
                }
            }
        }

        return 0;
    }
    //checks if a cordinate is out of bounds.
    fn oob(&mut self, x: usize, y: usize) -> bool {
        if x >= SIZE || y >= SIZE {
            println!("Move is out of bounds (Try again)");
            true
        } else { false }
    }

    //exception (override) doesnt matter what is there, does not implement position checking.
    pub fn set_tile(&mut self, x: usize, y: usize, value: u16, r#override: bool) {
        match self.get_tile(x, y) {
            0 => {
                *self.get_tile(x, y) = value;
            },
            _ => {
                if r#override {
                    *self.get_tile(x, y) = value;
                } else {
                    panic!("Tile is already occupied");
                }
            }
        }
    }

    pub fn int_to_char(&self, x: usize) -> char {
        let x_char:char = (x + 65) as u8 as char;
        x_char
    }

    pub fn print_board(&mut self) {
        for x in 0..SIZE {
            print!("  {} ", x + 1);
        } println!();
        for (row_i, row) in self.cells.iter().enumerate() {
            if row_i > 0 {
                println!(" {}---", "---+".repeat(SIZE-1));
            }
            print!("{}", self.int_to_char(row_i));
            for (cell_i, cell) in row.iter().enumerate() {
                if cell_i > 0 {
                    print!(" |");
                }

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