
pub struct OptionsOutput {
    pub file_for_log: String,
}

impl OptionsOutput {
    pub fn new() -> OptionsOutput {
        OptionsOutput {
            file_for_log: String::from("output_log.txt"),
        }
    }
}