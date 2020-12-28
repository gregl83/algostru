pub struct Store {
    pub welcomed: bool,
}

impl Store {
    pub fn new() -> Self {
        Store {
            welcomed: false,
        }
    }
}