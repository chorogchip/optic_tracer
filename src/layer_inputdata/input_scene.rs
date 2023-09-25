
use crate::layer_option::options;

pub struct InputScene {
    
}

impl InputScene {
}

pub fn read_scene(options: &options::Options) -> InputScene {
    if options.errors.has_serious_errors() {
        return InputScene{};
    }

    InputScene{ }
}