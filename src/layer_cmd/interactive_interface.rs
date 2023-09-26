
use std::io;
use std::io::Write;
use crate::layer_option::options;
use crate::layer_option::errors;

fn flush_output() {
    if let Err(error) = io::stdout().flush() {
        println!("error on flush: {error}");
    }
}

/*
fn read_val<T: std::str::FromStr>(error_val: T) -> T {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read line");
    return buf.trim().parse().unwrap_or(error_val);
}*/

fn read_string() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read line");
    return String::from(buf.trim());
}

fn get_val<T: std::str::FromStr + std::cmp::PartialOrd>(message: &str, min: T, max:T) -> T {
    loop {
        print!("{}>> ", message);
        flush_output();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("failed to read line");
        if let Ok(num) = buf.trim().parse() {
            if min <= num && num <= max {
                return num;
            }
        }
        print!("Wrong Input\n");
    }
}

fn get_string(message: &str) -> String {
    print!("{}>> ", message);
    flush_output();
    return read_string();
}

fn get_enum<T: std::str::FromStr>(message: &str) -> T {
    loop {
        if let Ok(en) = T::from_str(get_string(message).trim()) {
            return en;
        }
        print!("Wrong Input\n");
    }
}

pub fn execute_interactive(options: &mut options::Options) {

    print!("entered interactive mode\n");

    loop { match get_val(
"\n- Interactive Mode Options -
1) input file option
2) output file option
3) scene structure option
4) rendering option
5) performance option
6) complete and execute!
7) exit and cancel execution
", 1, 7) {
        1 => execute_interactive_input_file(options),
        2 => execute_interactive_output_file(options),
        3 => execute_interactive_scene_structure(options),
        4 => execute_interactive_rendering(options),
        5 => execute_interactive_performance(options),
        6 => {
            print!("> Completed interactive mod and Starting to Execute!\n");
            break;
        }
        7 => {
            options.errors.add_serious_error(errors::SeriousErrors::ExplicitExit(
                String::from("exit typed on interactive mod")));
            break;
        },
        _ => print!("Wrong input\n"),
    }}
}

fn execute_interactive_input_file(_options : &mut options::Options) {

    loop {  match get_val(
"\n- Input File Options -
0) go back
", 0, 0) {
        0 => break,
        _ => print!("Wrong input\n"),
    }}
}

fn execute_interactive_output_file(options : &mut options::Options) {
    
    loop { match get_val(
"\n- Output File Options -
0) go back
1) view output file options
2) log file name
3) log file type
4) image file name
5) image file type
6) image width
7) image height
", 0, 7) {
        0 => break,
        1 => print!("- View Output File Options -\n{}", options.options_output.to_string()),
        2 => options.options_output.log_file_name = get_string("Input file name to store log "),
        3 => options.options_output.log_file_type = get_enum("Input file type to store log "),
        4 => options.options_output.image_file_name = get_string("Input file name to store image "),
        5 => options.options_output.image_file_type = get_enum("Input file type to store image "),
        6 => options.options_output.height = get_val("Input image width ", 1, 32767),
        7 => options.options_output.width = get_val("Input image height ", 1, 32767),
        _ => print!("Wrong input\n"),
    }}
}

fn execute_interactive_scene_structure(_options : &mut options::Options) {

    loop { match get_val(
"\n- Scene Structure Options -
0) go back
", 0, 0) {
        0 => break,
        _ => print!("Wrong input\n"),
    }}
}

fn execute_interactive_rendering(_options : &mut options::Options) {
    
    loop { match get_val(
"\n- Rendering Options -
0) go back
", 0, 0) {
        0 => break,
        _ => print!("Wrong input\n"),
    }}
}

fn execute_interactive_performance(_options : &mut options::Options) {

    loop { match get_val(
"\n- Performance Options -
0) go back
", 0, 0) {
        0 => break,
        _ => print!("Wrong input\n"),
    }}
}

