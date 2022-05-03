use rand::{thread_rng, Rng};
use std::io;
use colored_truecolor::*;

// The size of the grid, X and Y
const GRID_X: usize = 8;
const GRID_Y: usize = 8;
const GRID_SIZE: usize = GRID_X * GRID_Y;

// The number of colors we are going to have
const COLORS_NUM: usize = 7;

#[derive(Clone, Copy, PartialEq)]
enum OwnedBy {
    Player,
    AI,
    Nobody
}

// The actual display of each of the color, where the index is the color represented by the u8
fn get_color(color_num: u8) -> ColoredString {
    match color_num {
        0 => "#".green(),
        1 => "#".red(),
        2 => "#".purple(),
        3 => "#".yellow(),
        4 => "#".blue(),
        5 => "#".cyan(),
        6 => "#".white(),
        _ => "?".white()
    }
}

#[derive(Clone, Copy)]
struct Block {
    pub color: u8,
    pub owned_by: OwnedBy
}

struct Filler {
    pub game_grid: [Block ; GRID_SIZE],
}

impl Filler {
    pub fn new() -> Filler {
        return Filler {
            game_grid: Filler::make_random_grid(),
        };
    }

    pub fn point_to_index(x: i32, y: i32) -> i32 {
        return y * GRID_X as i32 + x;
    }

    pub fn index_to_point(idx: i32) -> (i32, i32) {
        let y = (idx as f64 / GRID_X as f64) as i32;
        let x = idx % GRID_Y as i32;

        return (x, y);
    }

    pub fn get_neighbors(&self, x: i32, y: i32) -> Vec<Option<usize>> { // [Left, Right, Bottom, Top]
        let mut to_return: Vec<Option<usize>> = vec![];

        let left = Filler::point_to_index(x - 1, y);
        if left >= 0 && left < self.game_grid.len() as i32 {
            to_return.push(Some(left as usize));
        } else {
            to_return.push(None);
        }

        let right = Filler::point_to_index(x + 1, y);
        if right >= 0 && right < self.game_grid.len() as i32 {
            to_return.push(Some(right as usize));
        } else {
            to_return.push(None);
        }

        let bottom = Filler::point_to_index(x, y + 1);
        if bottom >= 0 && bottom < self.game_grid.len() as i32 {
            to_return.push(Some(bottom as usize));
        } else {
            to_return.push(None);
        }

        let top = Filler::point_to_index(x, y - 1);
        if top >= 0 && top < self.game_grid.len() as i32 {
            to_return.push(Some(top as usize));
        } else {
            to_return.push(None);
        }

        return to_return;
    }

    pub fn make_random_grid() -> [Block ; GRID_SIZE] {
        let mut rng = thread_rng();
        let mut to_return = [Block {color: 0, owned_by: OwnedBy::Nobody} ; GRID_SIZE];
    
        for num in 0..(GRID_SIZE- 1) {
            let r: u8 = rng.gen_range(0..COLORS_NUM) as u8;
            to_return[num].color = r;
        };
    
        return to_return;
    }

    pub fn render(&self) {
        let mut to_return = vec![];

        for (idx, element) in self.game_grid.iter().enumerate() {
            //to_return.push(element.color.to_string().chars().next().unwrap());
            to_return.push(get_color(element.color));

            if idx % GRID_X == 0 {
                to_return.insert(to_return.len() - 1, "\r\n".white());
            } else {
                to_return.insert(to_return.len() - 1, " ".white());
            }
        };

        for element in to_return {
            print!("{}", element);
        };

        println!();

        for color in 0..COLORS_NUM {
            let to_display = get_color(color as u8);
            println!("{} - {}", color, to_display);
        }
    }

    pub fn get_player_color(&self) -> u8 {
        return self.game_grid[Filler::point_to_index(0, GRID_Y as i32 - 1) as usize].color;
    }

    pub fn set_player_color(&mut self, new_color: u8) {
        for (idx, mut element) in self.game_grid.iter_mut().enumerate() {
            if element.owned_by == OwnedBy::Player {
                element.color = new_color;
            }
        }
    }

    pub fn get_ai_color(&self) -> u8 {
        return self.game_grid[Filler::point_to_index(GRID_X as i32 - 1, 0) as usize].color;
    }

    pub fn set_ai_color(&mut self, new_color: u8) {
        for (idx, mut element) in self.game_grid.iter_mut().enumerate() {
            if element.owned_by == OwnedBy::AI {
                element.color = new_color;
            }
        }
    }
}

fn main() {
    let mut game = Filler::new();
    game.game_grid[Filler::point_to_index(0, GRID_Y as i32 - 1) as usize].owned_by = OwnedBy::Player;
    game.game_grid[Filler::point_to_index(GRID_X as i32 - 1, 0) as usize].owned_by = OwnedBy::AI;

    loop {
        game.render();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(num) => {
                        println!("You chose: {}", num);
                        if num > COLORS_NUM as u8 {
                            continue;
                        }
                        if num == game.get_ai_color() || num == game.get_player_color() {
                            continue;
                        }

                        println!("Valid color!");

                        let mut neighbors_to_update: Vec<Vec<Option<usize>>> = vec![];
                        for (idx, mut element) in game.game_grid.iter_mut().enumerate() {
                            if element.owned_by == OwnedBy::Player {
                                println!("Changing {} to {}!", element.color, num);
                                element.color = num;
                                let cords = Filler::index_to_point(idx as i32);
                            }
                        }

                    },
                    Err(_) => continue
                }
            }
            Err(_) => continue
        };
    }
}
