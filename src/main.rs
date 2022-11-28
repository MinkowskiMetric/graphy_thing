use console_engine::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameSet {
    On,
    Off,
}

impl GameSet {
    fn as_scan(&self) -> KeyCode {
        match &self {
            GameSet::On => KeyCode::Char('A'),
            GameSet::Off => KeyCode::Char('B'),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            GameSet::On => "A",
            GameSet::Off => "B",
        }
    }
}

#[derive(Debug)]
struct GameCell { 
    op: GameSet,
    #[allow(dead_code)]
    focused: bool,
}

impl GameCell {
    pub fn new() -> Self {
        GameCell { op: GameSet::On, focused: false }
    }

    #[allow(dead_code)]
    fn get(&self) -> GameSet {
        self.op
    }

    #[allow(dead_code)]
    fn set(&mut self, op: GameSet) {
        self.op = op;
    }

    #[allow(dead_code)]
    fn get_focus(&self) -> bool {
        self.focused
    }

    #[allow(dead_code)]
    fn set_focus(&mut self, set_focus: bool) {
        self.focused = set_focus;
    }
}

#[derive(Debug)]
struct Input {
    key: GameSet,
    location: Option<GameCell>,
}

impl Input {
    pub fn setup(key: GameSet) -> Self {
        Self {
            key,
            location: None,
        }
    }

    pub fn on_key(&mut self, location: Option<GameCell>) {
        self.location = location;
    }
}

struct Game {
    dimensions: (usize, usize),
    data: Box<[GameCell]>,
    engine: console_engine::ConsoleEngine,
    inputs: Box<[Input]>,
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
        
        let inputs = vec![
            Input::setup(GameSet::On),
            Input::setup(GameSet::Off),
        ];

        let engine = console_engine::ConsoleEngine::init(20, 20, 3).unwrap();
        Game { dimensions, data: data.into_boxed_slice(), engine, inputs: inputs.into_boxed_slice(), }
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
            for input in self.inputs.iter_mut() {
                if self.engine.is_key_pressed(input.key.as_scan()) {
                    input.on_key(Some(GameCell { op: input.key, focused: false }));
                } else {
                    input.on_key(None);
                }
            }

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
                let scan = self.get((col as usize, r as usize));
                self.engine.print(x + 1 + col, y + 1 + r, scan.op.as_str().to_owned().as_str());
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
