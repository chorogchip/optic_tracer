
use crate::layer_option::options;
use super::parser;
use super::interactive_interface;

pub fn execute_cmds(input: parser::CommandInput) -> options::Options {
    let mut options = options::Options::new();
    let mut is_interactive_mode_enabled:bool = false;

    for arg in input.args {
        match arg {
            parser::EnumCommands::Interactive => is_interactive_mode_enabled = true,
            parser::EnumCommands::String(_str) => (),
        }
    }

    if is_interactive_mode_enabled {
        interactive_interface::execute_interactive(&mut options);
    }

    return options;
}