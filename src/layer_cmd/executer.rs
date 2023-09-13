
use super::{parser::{self, EnumCommands}, interactive_interface};

pub fn execute_cmds(input: parser::CommandInput) {
    let mut is_interactive_mode_enabled:bool = false;

    for arg in input.args {
        match arg {
            EnumCommands::Interactive => is_interactive_mode_enabled = true,
            EnumCommands::String(_str) => (),
        }
    }

    if is_interactive_mode_enabled {
        interactive_interface::execute_interactive();
    }
}