
use super::errors;

use super::options_output;

pub struct Options {
    pub errors: errors::Errors,

    pub options_output: options_output::OptionsOutput,
}

impl Options {
    pub fn new() -> Self {
        Options {
            errors: errors::Errors::new(),
            options_output: options_output::OptionsOutput::new(),
        }
    }
}



