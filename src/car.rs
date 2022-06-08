// Feel free to add fields as necessary
pub struct Car {
    name: String,
    path: Vec<(usize, usize)>,
}

impl Car {
    pub fn new(name: &str, path: Vec<(usize, usize)>) -> Self {
        Car {
            name: String::from(name),
            path,
        }
    }

    // Hint: One approach is to have an async method here taking care of a single car's
    // race and communicating with the city when necessary.
}
