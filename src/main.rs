use rand::{thread_rng, Rng};
use colored_truecolor::*;

// The size of the grid, X and Y
const GRID_X: usize = 8;
const GRID_Y: usize = 8;
const GRID_SIZE: usize = GRID_X * GRID_Y;

// The number of colors we are going to have
const COLORS_NUM: usize = 7;

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
    pub owned_by: bool // True if owned by player, false if owned by ai
}

struct Filler {
    game_grid: [Block ; GRID_SIZE],
}

impl Filler {
    pub fn new() -> Filler {
        return Filler {
            game_grid: make_random_grid(),
        };
    }

    pub fn render(self) {
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
    }
}

fn make_random_grid() -> [Block ; GRID_SIZE] {
    let mut rng = thread_rng();
    let mut to_return = [Block {color: 0, owned_by: true} ; GRID_SIZE];

    for num in 0..(GRID_SIZE- 1) {
        let r: u8 = rng.gen_range(0..COLORS_NUM) as u8;
        to_return[num].color = r;
    };

    return to_return;
}

fn main() {
    let game = Filler::new();
    game.render();
    
}
