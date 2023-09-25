
pub mod layer_math;

pub mod layer_option;
pub mod layer_cmd;
pub mod layer_render;
pub mod layer_inputdata;
pub mod layer_scenedata;
pub mod layer_outputdata;
pub mod layer_store;

fn main() {
    let parsed_cmds = layer_cmd::parser::parse_input();
    let mut options = layer_cmd::executer::execute_cmds(parsed_cmds);

    let input_scene = layer_inputdata::input_scene::read_scene(&mut options);
    let processed_scene = layer_scenedata::processed_scene::process_scene(input_scene, &mut options);
    let rendered_data = layer_render::renderer::render_scene(processed_scene, &mut options);
    let output_data = layer_outputdata::output_data::process_output(rendered_data, &mut options);
    let store_result = layer_store::store_manager::store_data(output_data, &mut options);

    store_result.print();
}
