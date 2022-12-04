use console_engine::{KeyCode, events::Event, crossterm::event::MouseEvent};
use std::{cell::RefCell, fmt};

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
    engine: ConsoleGraph,
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

        PlayBook { dimensions, data: data.into_boxed_slice(), engine: ConsoleGraph::init(20, 25, 3).unwrap(), }
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

            Event::Mouse(mouse) => {
                let pos = (mouse.column as usize, mouse.row as usize);
                if pos.0 < self.dimensions.0 && pos.1 < self.dimensions.1 {
                    self.on_mouse(&self.data[(self.dimensions.0 * pos.1) * pos.0], pos, mouse)
                } else {
                    None
                }
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

    fn on_mouse(&self, _play_room_cell: &PlayBookCell, _pos: (usize, usize), _mouse_event: MouseEvent) -> Option<SingleCommand> {
        todo!()
    }

    fn print_code(&self, x: i32, y: i32, string: &str) {
        self.engine.print(x, y, string);
    }

    fn print_glyph(&self, x: i32, y: i32, shape: GameCommand) {
        self.print_code(x, y, shape.to_string().as_str());
    }

    pub fn print(&self, x: i32, y: i32) {
        let (mul_col, mul_row) = (self.dimensions.0 as i32, self.dimensions.1 as i32);

        self.print_code(x, y, "+");
        for col in 0..mul_col {
            self.print_code(x + 1 + col, y, "-");
        }
        self.print_code(x + 1 + mul_col, y, "+");

        for col in 0..mul_col {
            self.print_code(x, y + 1 + col, "|");
            for row in 0..mul_row {
                self.print_glyph(x + 1 + col, y + 1 + row, self.data[col as usize].get_visual());
            }
            self.print_code(x + 1 + mul_col, y + 1 + col, "|");
        }
        
        self.print_code(x, y + 1 + mul_row, "+");
        for col in 0..mul_col {
            self.print_code(x + 1 + col, y + 1 + mul_row, "-");
        }
        self.print_code(x + 1 + mul_col, y + 1 + mul_row, "+");
    }
}

fn main() {
    let dimensions = (15, 15);
    PlayBook::from_dimensions(dimensions).play();
}
