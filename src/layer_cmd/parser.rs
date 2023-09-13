

pub enum EnumCommands {
   String(String),

   Interactive,
}

pub struct CommandInput {
    pub args: Vec<EnumCommands>,
}


pub fn parse_input() -> CommandInput {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let mut ret = CommandInput {
        args: Vec::new(),
    };

    for arg in args {
        ret.args.push(
            match arg.as_ref() {
                "-intr" => EnumCommands::Interactive,
                _ => EnumCommands::String(arg),
            }
        );
    }

    return ret;
}