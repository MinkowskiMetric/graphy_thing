use console_engine::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameCommand {
    #[allow(dead_code)]
    On,
}

struct PlayBookCell {

}

impl PlayBookCell {
    pub fn new() -> Self {
        PlayBookCell { }
    }
}

struct PlayBook {
    dimensions: (usize, usize),
    #[allow(dead_code)]
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
        for _ in 0..(dimensions.0 * dimensions.1) {
            data.push(PlayBookCell::new());
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

    pub fn print(&mut self, x: i32, y: i32) {
        let (mul_col, mul_row) = (self.dimensions.0 as i32, self.dimensions.1 as i32);

        self.engine.print(x, y, "+");
        for col in 0..mul_col {
            self.engine.print(x + 1 + col, y, "-");
        }
        self.engine.print(x + 1 + mul_col, y, "+");

        for col in 0..mul_col {
            self.engine.print(x, y + 1 + col, "|");
            for _row in 0..mul_row {
                // TODO self.engine.print(x + 1 + col, y + 1 + r, self.cell_layout((col as usize, row as usize)).as_scan());
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
