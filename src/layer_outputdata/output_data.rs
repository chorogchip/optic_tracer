
use crate::layer_option::options;
use crate::layer_render::output_raw_data;

pub struct OutputData {

}

impl OutputData {

}

pub fn process_output(_raw_data: output_raw_data::OutputRawData, options: &mut options::Options) -> OutputData {
    if options.errors.has_serious_errors() {
        return OutputData{};
    }
    return OutputData{ };
}
