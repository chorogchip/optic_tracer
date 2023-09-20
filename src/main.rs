
pub mod layer_math;

pub mod layer_cmd;
pub mod layer_render;
pub mod layer_inputdata;
pub mod layer_scenedata;
pub mod layer_outputdata;
pub mod layer_store;

fn main() {
    let parsed_cmds = layer_cmd::parser::parse_input();
    let options = layer_cmd::executer::execute_cmds(parsed_cmds);

    if (options.has_serious_errors()) {
        println!("asdasd");
    }

    let input_scene = layer_inputdata::input_scene::read_scene(&options);
    let processed_scene = layer_scenedata::processed_scene::process_scene(input_scene, &options);
    let rendered_data = layer_render::renderer::render_scene(processed_scene, &options);
    let output_data = layer_outputdata::output_data::process_output(rendered_data, &options);
    let store_result = layer_store::store_manager::store_data(output_data, &options);
    store_result.print();
}
