
pub mod layer_cmd;

fn main() {
    let parsed_cmds = layer_cmd::parser::parse_input();
    layer_cmd::executer::execute_cmds(parsed_cmds);
    
}
