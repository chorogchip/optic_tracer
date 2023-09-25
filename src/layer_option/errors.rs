
pub struct Errors {
    serious_errors: Vec<SeriousErrors>,
    trivial_errors: Vec<TrivialErrors>,
}

impl Errors {
    pub fn new() -> Self {
        Errors {
            serious_errors: Vec::new(),
            trivial_errors: Vec::new()
        }
    }

    pub fn has_serious_errors(&self) -> bool {
        self.serious_errors.len() > 0
    }
    pub fn has_trivial_errors(&self) -> bool {
        self.trivial_errors.len() > 0
    }
    pub fn add_serious_error(&mut self, error: SeriousErrors) {
        self.serious_errors.push(error);
    }
    pub fn add_trivial_error(&mut self, error: TrivialErrors) {
        self.trivial_errors.push(error);
    }
    pub fn get_serious_error_vec(&self) -> &Vec<SeriousErrors> {
        &self.serious_errors
    }
    pub fn get_trivial_error_vec(&self) -> &Vec<TrivialErrors> {
        &self.trivial_errors
    }
}

pub enum SeriousErrors {
    ExplicitExit(String),
}

pub enum TrivialErrors {
    // in store manager, have to add match arm
}

