use std::fmt;
use console_engine::KeyCode;

#[derive(Debug)]
struct GameCell {

}

impl fmt::Display for GameCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", 0)
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
            data.push(GameCell {});
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
                self.engine.print(x + 1 + col, y + 1 + r, "0");
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

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let col = self.dimensions.0;

        write!(f, "+")?;
        for _ in 0..col {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;

        for col in 0..col {
            write!(f, "|")?;
            for row in 0..self.dimensions.1 {
                self.get((col, row)).fmt(f)?;
            }
            writeln!(f, "|")?;
        }

        write!(f, "+")?;
        for _ in 0..col {
            write!(f, "-")?;
        }
        write!(f, "+")?;

        Ok(())
    }
}

fn main() {
    let dimensions = (15, 15);
    Game::from_dimensions(dimensions).play();
}
