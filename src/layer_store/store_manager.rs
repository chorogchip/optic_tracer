
use crate::layer_option::errors::SeriousErrors;
use crate::layer_option::options;
use crate::layer_outputdata::output_data::OutputData;

pub struct StoreResult {
    message: String,
}

impl StoreResult {
    pub fn new() -> StoreResult {
        StoreResult{
            message: String::from("- Final Results -\n"),
        }
    }
    pub fn print(&self) {
        let message = &self.message;
        print!("\n{}\n", message);
    }
}

pub fn store_data(_output_data: OutputData, options: &mut options::Options) -> StoreResult {

    let mut ret = StoreResult::new();
    let mut has_error_occured = false;

    if options.errors.has_serious_errors() {
        has_error_occured = true;

        let error_vec = options.errors.get_serious_error_vec();
        let error_cnt = error_vec.len();

        ret.message.push_str(&format!("Error: there were {error_cnt} serious errors.\n    Errors:\n"));
        for err in error_vec {
            ret.message.push_str(&match err {
                SeriousErrors::ExplicitExit(str) => format!("    - Explicit Exit {{{str}}}\n"),
            });
        }
    }

    if options.errors.has_trivial_errors() { 
        has_error_occured = true;

        let error_vec = options.errors.get_trivial_error_vec();
        let error_cnt = error_vec.len();

        ret.message.push_str(&format!("Error - there were {error_cnt} trivial errors.\n    Errors :\n"));
        for err in error_vec {
            ret.message.push_str(&match err {
                _ => String::new(),  // has to be removed when trivial error has added
            });
        }
    }

    if !has_error_occured {
        ret.message.push_str("> Successfully executed without any errors\n");
    }

    return ret;
}

