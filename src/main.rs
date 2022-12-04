use console_engine::{KeyCode, events::Event};
use std::{cell::RefCell, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn from_sizes(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }
}

struct ConsoleGraph {
    e: RefCell<console_engine::ConsoleEngine>,
}

impl ConsoleGraph {
    pub fn init(width: u32, height: u32, target_fps: u32) -> Result<Self, std::io::Error> {
        match console_engine::ConsoleEngine::init(width, height, target_fps) {
            Ok(e) => Ok(Self { e: RefCell::new(e) }),
            Err(e) => Err(e),
        }
    }

    pub fn poll(&self) -> Event {
        self.e.borrow_mut().poll()
    }

    pub fn clear_screen(&self) {
        self.e.borrow_mut().clear_screen();
    }

    pub fn draw(&self) {
       self.e.borrow_mut().draw();
    }

    pub fn print(&self, x: i32, y: i32, string: &str) {
        self.e.borrow_mut().print(x, y, string)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SingleCommand {
    Quit,
}

impl fmt::Display for SingleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SingleCommand::Quit => f.write_str("Quit"),
        }
    }    
}

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
    point: Size,
    command: GameCommand,
}

impl PlayBookCell {
    pub fn new(point: Size) -> Self {
        PlayBookCell { 
            point,
            command: GameCommand::Off,
        }
    }

    #[allow(dead_code)]
    pub fn get_visual(&self) -> GameCommand {
        self.command
    }
}

struct PlayBook {
    dimensions: Size,
    data: Box<[Box<[PlayBookCell]>]>,
    engine: ConsoleGraph,
}

impl PlayBook {
    #[allow(dead_code)]
    pub fn from_sizes(width: usize, height: usize) -> Self {
        Self::from_dimensions(Size::from_sizes(width, height))
    }

    pub fn from_dimensions(dimensions: Size) -> Self {
        let mut screen_data = Vec::with_capacity(dimensions.get_width());
        for x in 0..dimensions.get_width() {
            let mut screen_height = Vec::with_capacity(dimensions.get_height());
            for y in 0..dimensions.get_height() {
                screen_height.push(PlayBookCell::new(Size::from_sizes(x, y)));
            }
            screen_data.push(screen_height.into_boxed_slice());
        }
        
        PlayBook { dimensions, data: screen_data.into_boxed_slice(), engine: ConsoleGraph::init(30, 30, 3).unwrap(), }
    }

    fn single_input(&self) -> Option<SingleCommand> {
        match self.engine.poll() {
            Event::Frame => {
                self.engine.clear_screen();
                self.print(0, 0);
                self.engine.draw();
                None
            },

            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => Some(SingleCommand::Quit),
                    _ => None,
                }
            },

            Event::Mouse(_mouse) => {
                None
            },

            Event::Resize(_width, _heights) => {
                None
            },
        }
    }

    pub fn play(self) {
        while self.single_input().is_none() {
        }
    }

    fn print_code(&self, x: i32, y: i32, string: &str) {
        self.engine.print(x, y, string);
    }

    fn print_glyph(&self, x: i32, y: i32, shape: GameCommand) {
        self.print_code(x, y, shape.to_string().as_str());
    }

    pub fn print(&self, x: i32, y: i32) {
        let (mul_col, mul_row) = (self.dimensions.get_width() as i32, self.dimensions.get_height() as i32);

        self.print_code(x, y, "+");
        for col in 0..mul_col {
            self.print_code(x + 1 + col, y, "-");
        }
        self.print_code(x + 1 + mul_col, y, "+");

        for row in 0..mul_row {
            self.print_code(x, y + 1 + row, "|");
            for col in 0..mul_col{
                self.print_glyph(x + 1 + col, y + 1 + row, self.data[x as usize][y as usize].command);
            }
            self.print_code(x + 1 + mul_col, y + 1 + row, "|");
        }
        
        self.print_code(x, y + 1 + mul_row, "+");
        for col in 0..mul_col {
            self.print_code(x + 1 + col, y + 1 + mul_row, "-");
        }
        self.print_code(x + 1 + mul_col, y + 1 + mul_row, "+");
    }
}

fn main() {
    PlayBook::from_sizes(25, 10).play();
}
