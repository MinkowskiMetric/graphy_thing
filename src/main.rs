use console_engine::{
    crossterm::event::{KeyEvent, MouseEvent, MouseEventKind},
    events::Event,
    KeyCode, MouseButton,
};
use core::str;
use std::{fmt, time::Instant};

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

    pub fn to_rectangle(&self) -> Rectangle {
        Rectangle {
            left: 0,
            top: 0,
            right: self.width,
            bottom: self.height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl Rectangle {
    pub fn from_sizes(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn get_left(&self) -> usize {
        self.left
    }

    pub fn set_left(&mut self, left: usize) {
        self.left = left;
    }

    pub fn get_top(&self) -> usize {
        self.top
    }

    pub fn set_top(&mut self, top: usize) {
        self.top = top;
    }

    pub fn get_right(&self) -> usize {
        self.right
    }

    pub fn set_right(&mut self, right: usize) {
        self.right = right;
    }

    pub fn get_bottom(&self) -> usize {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: usize) {
        self.bottom = bottom;
    }

    pub fn expand(
        &self,
        left_offset: isize,
        top_offset: isize,
        right_offset: isize,
        bottom_offset: isize,
    ) -> Rectangle {
        Self {
            left: self.left - left_offset as usize,
            top: self.top - top_offset as usize,
            right: self.right + right_offset as usize,
            bottom: self.bottom + bottom_offset as usize,
        }
    }
}

struct ConsoleGraph {
    e: console_engine::ConsoleEngine,
}

impl ConsoleGraph {
    pub fn init(width: u32, height: u32, target_fps: u32) -> Result<Self, std::io::Error> {
        match console_engine::ConsoleEngine::init(width, height, target_fps) {
            Ok(e) => Ok(Self { e }),
            Err(e) => Err(e),
        }
    }

    pub fn poll(&mut self) -> Event {
        self.e.poll()
    }

    pub fn wait_frame(&mut self) {
        self.e.wait_frame()
    }

    pub fn clear_screen(&mut self) {
        self.e.clear_screen();
    }

    pub fn draw(&mut self) {
        self.e.draw();
    }

    pub fn print(&mut self, x: i32, y: i32, string: &str) {
        self.e.print(x, y, string)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SingleCommand {
    Switch,
    Quit,
}

impl fmt::Display for SingleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SingleCommand::Switch => f.write_str("Switch"),
            SingleCommand::Quit => f.write_str("Quit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameCommand {
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
    timer: Option<Instant>,
    command: GameCommand,
}

impl PlayBookCell {
    pub fn new(point: Size) -> Self {
        PlayBookCell {
            point,
            timer: None,
            command: GameCommand::Off,
        }
    }

    fn action(&mut self) {
        self.timer = Some(Instant::now());
        self.command = GameCommand::On;
    }

    fn clear_setup(&mut self) {
        if self
            .timer
            .as_ref()
            .and_then(|instant| {
                if instant.elapsed().as_secs() >= 2 {
                    Some(())
                } else {
                    None
                }
            })
            .is_some()
        {
            self.timer = None;
            self.command = GameCommand::Off;
        }
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

        PlayBook {
            dimensions,
            data: screen_data.into_boxed_slice(),
            engine: ConsoleGraph::init(30, 30, 25).unwrap(),
        }
    }

    #[allow(dead_code)]
    fn get_col(&self, col: usize) -> &[PlayBookCell] {
        &self.data[col]
    }

    #[allow(dead_code)]
    fn get_row(&self, col: usize, row: usize) -> &PlayBookCell {
        &self.get_col(col)[row]
    }

    fn get_col_mut(&mut self, col: usize) -> &mut [PlayBookCell] {
        &mut self.data[col]
    }

    fn get_row_mut(&mut self, col: usize, row: usize) -> &mut PlayBookCell {
        &mut self.get_col_mut(col)[row]
    }

    fn single_input(&mut self) -> Option<SingleCommand> {
        match self.engine.poll() {
            Event::Frame => {
                self.engine.clear_screen();
                self.clear_setup();
                self.engine.wait_frame();
                self.print(0, 0);
                self.engine.draw();
                None
            }

            Event::Key(key) => self.handle_key(&key),
            Event::Mouse(mouse) => self.handle_mouse(&mouse),

            Event::Resize(_width, _heights) => None,
        }
    }

    fn clear_setup(&mut self) {
        for col in 0..self.dimensions.get_width() {
            for row in 0..self.dimensions.get_height() {
                self.data[col][row].clear_setup();
            }
        }
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<SingleCommand> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => Some(SingleCommand::Quit),
            _ => None,
        }
    }

    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<SingleCommand> {
        let (mouse_row, mouse_column) = (mouse.row as usize, mouse.column as usize);
        if (mouse.kind == MouseEventKind::Moved
            || mouse.kind == MouseEventKind::Down(MouseButton::Left)
            || mouse.kind == MouseEventKind::Up(MouseButton::Left))
            && mouse_column >= 1
            && mouse_column <= self.dimensions.width
            && mouse_row >= 1
            && mouse_row <= self.dimensions.height
        {
            let (col, row) = (mouse_column - 1, mouse_row - 1);
            let play_book_cell = self.get_row_mut(col, row);
            play_book_cell.action();
            Some(SingleCommand::Switch)
        } else {
            None
        }
    }

    pub fn play(mut self) {
        loop {
            match self.single_input() {
                None | Some(SingleCommand::Switch) => (),
                Some(SingleCommand::Quit) => {
                    return;
                }
            }
        }
    }

    fn print_code(&mut self, x: i32, y: i32, string: &str) {
        self.engine.print(x, y, string);
    }

    fn print_glyph(&mut self, x: i32, y: i32, shape: GameCommand) {
        self.print_code(x, y, shape.to_string().as_str());
    }

    fn print_horiz(&mut self, x1: i32, x2: i32, y: i32, stag: &str, ttag: &str, etag: &str) {
        let len = (x2 - x1).max(2);
        let result = stag.to_owned() + &ttag.repeat(len as usize - 2) + etag;
        self.print_code(x1, y, result.as_str());
    }

    pub fn print(&mut self, x: i32, y: i32) {
        let (mul_col, mul_row) = (
            self.dimensions.get_width() as i32,
            self.dimensions.get_height() as i32,
        );

        for row in 0..mul_row {
            self.print_code(x, y + 1 + row, "|");
            for col in 0..mul_col {
                self.print_glyph(
                    x + 1 + col,
                    y + 1 + row,
                    self.data[(x + col) as usize][(y + row) as usize].command,
                );
            }
            self.print_code(x + 1 + mul_col, y + 1 + row, "|");
        }

        self.print_horiz(x, x + 2 + mul_col, y, "+", "-", "+");
        self.print_horiz(x, x + 2 + mul_col, y + 1 + mul_row, "+", "-", "+");
    }
}

fn main() {
    PlayBook::from_sizes(25, 10).play();
}
