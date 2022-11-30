use console_engine::KeyCode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameCommand {
    #[allow(dead_code)]
    On,
    Off,
}

impl fmt::Display for GameCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameCommand::On => f.write_str("O"),
            GameCommand::Off => f.write_str("."),
        }
    }
}

struct PlayBookCell {
    #[allow(dead_code)]
    point: (usize, usize),
    command: GameCommand,
}

impl PlayBookCell {
    pub fn new(point: (usize, usize)) -> Self {
        PlayBookCell { 
            point,
            command: GameCommand::Off,
        }
    }

    pub fn get_visual(&self) -> GameCommand {
        self.command
    }
}

struct PlayBook {
    dimensions: (usize, usize),
    data: Box<[PlayBookCell]>,
    engine: console_engine::ConsoleEngine,
}

impl PlayBook {
    #[allow(dead_code)]
    pub fn from_sizes(width: usize, height: usize) -> Self {
        Self::from_dimensions((width, height))
    }

    pub fn from_dimensions(dimensions: (usize, usize)) -> Self {
        let mut data = Vec::with_capacity(dimensions.0 * dimensions.1);
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                data.push(PlayBookCell::new((x, y)));
            }
        }

        let engine = console_engine::ConsoleEngine::init(20, 20, 3).unwrap();
        PlayBook { dimensions, data: data.into_boxed_slice(), engine, }
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

    fn print_glyph(&mut self, x: i32, y: i32, shape: GameCommand) {
        self.engine.print(x, y, &shape.to_owned().to_string());
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
            for row in 0..mul_row {
                self.print_glyph(x + 1 + col, y + 1 + row, self.data[col as usize].get_visual());
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
    PlayBook::from_dimensions(dimensions).play();
}
