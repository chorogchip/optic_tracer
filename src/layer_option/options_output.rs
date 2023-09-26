
pub struct OptionsOutput {
    pub file_for_log: String,
}

impl OptionsOutput {
    pub fn new() -> OptionsOutput {
        OptionsOutput {
            file_for_log: String::from("output_log.txt"),
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("file for log: {}\n", self.file_for_log));
        return buf;
    }
}