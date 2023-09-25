

use crate::layer_option::options;
use crate::layer_scenedata::processed_scene;
use super::output_raw_data;

pub fn render_scene(_scene: processed_scene::ProcessedScene, options: &mut options::Options) -> output_raw_data::OutputRawData {
    if options.errors.has_serious_errors() {
        return output_raw_data::OutputRawData{};
    }
    return output_raw_data::OutputRawData{ };
}