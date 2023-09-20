
use std;

pub struct Options {
    errors: Errors,
}

impl Options {
    pub fn new() -> Self {
        Options {
            errors: Errors::new(),
        }
    }

    pub fn has_serious_errors(&self) -> bool {
        self.errors.serious_errors.len() > 0
    }
    pub fn add_serious_error(&mut self, error: SeriousErrors) {
        self.errors.serious_errors.push(error);
    }
    pub fn add_trivial_error(&mut self, error: TrivialErrors) {
        self.errors.trivial_errors.push(error);
    }
    pub fn get_serious_error_vec(&self) -> &Vec<SeriousErrors> {
        &self.errors.serious_errors
    }
    pub fn get_trivial_error_vec(&self) -> &Vec<TrivialErrors> {
        &self.errors.trivial_errors
    }
}


struct Errors {
    serious_errors: Vec<SeriousErrors>,
    trivial_errors: Vec<TrivialErrors>,
}

impl Errors {
    fn new() -> Self {
        Errors {
            serious_errors: Vec::new(),
            trivial_errors: Vec::new()
        }
    }
}

pub enum SeriousErrors {
    ExplicitExit(String),
}

pub enum TrivialErrors {

}


