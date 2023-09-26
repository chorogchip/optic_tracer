
pub struct OptionsOutput {
    pub log_file_name: String,
    pub log_file_type: LogFileType,
    pub image_file_name: String,
    pub image_file_type: ImageFileType,
    pub width: i32,
    pub height: i32,
}

impl OptionsOutput {
    pub fn new() -> OptionsOutput {
        OptionsOutput {
            log_file_name: String::from("output_log"),
            log_file_type: LogFileType::TXT,
            image_file_name: String::from("output_image"),
            image_file_type: ImageFileType::PPM,
            width: 256,
            height: 144,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("log file name: {}\n", self.log_file_name));
        buf.push_str(&format!("log file type: {}\n", self.log_file_type.to_string()));
        buf.push_str(&format!("image file name: {}\n", self.image_file_name));
        buf.push_str(&format!("image file type: {}\n", self.image_file_type.to_string()));
        buf.push_str(&format!("image width: {}\n", self.width));
        buf.push_str(&format!("image height: {}\n", self.height));
        return buf;
    }
}

pub enum LogFileType {
    TXT,
    LOG,
}

impl LogFileType {
    pub fn to_string(&self) -> String {
        match &self {
            LogFileType::TXT => String::from("txt"),
            LogFileType::LOG => String::from("log"),
        }
    }
}

impl std::str::FromStr for LogFileType {
    type Err = String;
    
    fn from_str(input: &str) -> Result<LogFileType, String> {
        match input {
            "txt" => Ok(LogFileType::TXT),
            "log" => Ok(LogFileType::LOG),
            _ => Err(String::from(input)),
        }
    }
}

pub enum ImageFileType {
    PPM,
}

impl ImageFileType {
    pub fn to_string(&self) -> String {
        match &self {
            ImageFileType::PPM => String::from("ppm"),
        }
    }
}

impl std::str::FromStr for ImageFileType {
    type Err = String;

    fn from_str(input: &str) -> Result<ImageFileType, Self::Err> {
        match input.to_lowercase().as_str() {
            "ppm" => Ok(ImageFileType::PPM),
            _ => Err(String::from(input)),
        }
    }
}
