

use crate::layer_inputdata::input_scene;
use crate::layer_option::options;

pub struct ProcessedScene {

}

impl ProcessedScene {
}

pub fn process_scene(_input_data: input_scene::InputScene, options: &mut options::Options) -> ProcessedScene {
    if options.errors.has_serious_errors() {
        return ProcessedScene{ };
    }
    return ProcessedScene{ };
}