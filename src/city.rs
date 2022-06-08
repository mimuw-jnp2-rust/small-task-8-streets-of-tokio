#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
}

// Feel free to add and/or change fields as necessary
#[derive(Clone)]
pub struct Lights {
    color: Color,
    duration: u64,
}

impl Lights {
    // This method should produce traffic lights initially
    // set to green
    pub fn green(duration: u64) -> Self {
        todo!()
    }

    // This method should produce traffic lights initially
    // set to red
    pub fn red(duration: u64) -> Self {
        todo!()
    }
}

// Feel free to add and/or change fields as necessary
pub struct City {
    width: usize,
    height: usize,
    lights: Vec<Option<Lights>>,
}

impl City {
    pub fn new(width: usize, height: usize) -> Self {
        todo!()
    }

    pub fn add_lights(&mut self, x: usize, y: usize, lights: Lights) {
        todo!()
    }
}
