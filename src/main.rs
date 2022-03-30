use rand::{thread_rng, Rng};

// The size of the grid, X and Y
const GRID_X: usize = 8;
const GRID_Y: usize = 7;
const GRID_SIZE: usize = GRID_X * GRID_Y;

// The number of colors we are going to have
const COLORS_NUM: usize = 7;

pub struct Filler {
    game_grid: [u8 ; GRID_SIZE]
}

impl Filler {
    pub fn new() -> Filler {
        return Filler {
            game_grid: make_random_grid()
        };
    }

    pub fn render(self) -> String {
        let mut to_return = String::new();

        for element in self.game_grid {
            to_return.push(element.to_string().chars().next().unwrap());
        };

        for row in 1..GRID_Y + 1 {
            to_return.insert_str(GRID_X * row, "\r\n");
        };

        return to_return;
    }
}

fn make_random_grid() -> [u8 ; GRID_SIZE] {
    let mut rng = thread_rng();
    let mut to_return: [u8 ; GRID_SIZE] = [0 ; GRID_SIZE];

    for num in 0..(GRID_SIZE- 1) {
        let r: u8 = rng.gen_range(0..COLORS_NUM) as u8;
        to_return[num] = r;
    };

    return to_return;
}

fn main() {
    let game = Filler::new();
    println!("{}", game.render());
}
