use console_engine::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameSet {
    On,
    #[allow(dead_code)]
    Off,
}

impl GameSet {
    fn as_scan(&self) -> &str {
        match self {
            GameSet::On => "A",
            GameSet::Off => "B",
        }
    }
}

#[derive(Debug)]
struct GameCell { 
    op: GameSet,
}

impl GameCell {
    pub fn new() -> Self {
        GameCell { op: GameSet::On }
    }

    fn get(&self) -> GameSet {
        self.op
    }

    #[allow(dead_code)]
    fn set(&mut self, op: GameSet) {
        self.op = op;
    }
}

struct Game {
    dimensions: (usize, usize),
    data: Box<[GameCell]>,
    engine: console_engine::ConsoleEngine,
}

impl Game {
    #[allow(dead_code)]
    pub fn from_sizes(width: usize, height: usize) -> Self {
        Self::from_dimensions((width, height))
    }

    pub fn from_dimensions(dimensions: (usize, usize)) -> Self {
        let mut data = Vec::with_capacity(dimensions.0 * dimensions.1);
        for _ in 0..(dimensions.0 * dimensions.1) {
            data.push(GameCell::new());
        }
        
        let engine = console_engine::ConsoleEngine::init(20, 20, 3).unwrap();
        Game { dimensions, data: data.into_boxed_slice(), engine }
    }

    pub fn get(&self, pt: (usize, usize)) -> &GameCell {
        &self.data[(self.dimensions.0 * pt.0) + pt.1]
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, pt: (usize, usize)) -> &mut GameCell {
        &mut self.data[(self.dimensions.0 * pt.0) + pt.1]
    }

    fn single_input(&mut self) -> Option<()> {
        self.engine.wait_frame();
        self.engine.clear_screen();

        if self.engine.is_key_pressed(KeyCode::Char('q')) {
            None
        } else {
            self.print(0, 0);
            self.engine.draw();
            Some(())
        }
    }

    pub fn play(mut self) {
        while self.single_input() == Some(()) {
        }
    }

    pub fn print(&mut self, x: i32, y: i32) {
        let (mul_col, mul_row) = (self.dimensions.0 as i32, self.dimensions.1 as i32);

        self.engine.print(x, y, "+");
        for col in 0..mul_col {
            self.engine.print(x + 1 + col, y, "-");
        }
        self.engine.print(x + 1 + mul_col, y, "+");

        for col in 0..mul_col {
            self.engine.print(x, y + 1 + col, "|");
            for r in 0..mul_row {
                let scan = self.get((col as usize, r as usize)).get().to_owned();
                self.engine.print(x + 1 + col, y + 1 + r, scan.as_scan());
            }
            self.engine.print(x + 1 + mul_col, y + 1 + col, "|");
        }
        
        self.engine.print(x, y + 1 + mul_row, "+");
        for col in 0..mul_col {
            self.engine.print(x + 1 + col, y + 1 + mul_row, "-");
        }
        self.engine.print(x + 1 + mul_col, y + 1 + mul_row, "+");
    }
}

fn main() {
    let dimensions = (15, 15);
    Game::from_dimensions(dimensions).play();
}
