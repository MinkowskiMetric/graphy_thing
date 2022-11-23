use std::fmt;

#[derive(Debug)]
struct GameCell {

}

impl fmt::Display for GameCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", 0)
    }
}

#[derive(Debug)]
struct Game {
    dimensions: (usize, usize),
    data: Box<[GameCell]>,
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
        println!{"size: {}", dimensions.0 * dimensions.1};
        Game { dimensions, data: data.into_boxed_slice() }
    }

    pub fn get(&self, pt: (usize, usize)) -> &GameCell {
        &self.data[(self.dimensions.0 * pt.0) + pt.1]
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, pt: (usize, usize)) -> &mut GameCell {
        &mut self.data[(self.dimensions.0 * pt.0) + pt.1]
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
    let game = Game::from_dimensions(dimensions);

    println!("Hello, world!");
    println!("Box:\n{}", game);
}
