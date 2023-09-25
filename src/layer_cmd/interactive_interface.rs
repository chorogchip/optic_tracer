
use std::io;
use std::io::Write;
use crate::layer_option::options;
use crate::layer_option::errors;

fn read_int() -> i64 {
    let mut buf = String::new();
    loop {
        io::stdin().read_line(&mut buf).expect("failed to read line");
        let input_int:i64 = buf.trim().parse().expect("please input an integer");
        if input_int >= 1 && input_int <= 7 {
            break input_int;
        }
    }
}

fn flush_output() {
    if let Err(error) = io::stdout().flush() {
        println!("error on flush: {error}");
    }
}

pub fn execute_interactive(options: &mut options::Options) {

    print!("entered interactive mode\n");

    loop {

        print!(
"\n- Interactive Mode Options -
1) input file option
2) output file option
3) scene structure option
4) rendering option
5) performance option
6) complete and execute!
7) exit and cancel execution
>> ");
        flush_output();
    
        match read_int() {
            1 => execute_interactive_input_file(options),
            2 => execute_interactive_output_file(options),
            3 => execute_interactive_scene_structure(options),
            4 => execute_interactive_rendering(options),
            5 => execute_interactive_performance(options),
            6 => {
                print!("Completed interactive mod and Starting to Execute!\n");
                break;
            }
            7 => {
                options.errors.add_serious_error(errors::SeriousErrors::ExplicitExit(
                    String::from("exit typed on interactive mod")));
                break;
            },
            _ => print!("Wrong input\n"),
        }
    }

}

fn execute_interactive_input_file(options : &mut options::Options) {

    loop {

        print!(
"\n- Input File Options -
1) go back
>> ");

        flush_output();
            
        match read_int() {
            1 => break,
            _ => print!("Wrong input\n"),
        }
    }
}

fn execute_interactive_output_file(options : &mut options::Options) {
    loop {

        print!(
"\n- Output File Options -
1) go back
>> ");

        flush_output();
            
        match read_int() {
            1 => break,
            _ => print!("Wrong input\n"),
        }
    }
}

fn execute_interactive_scene_structure(options : &mut options::Options) {
    loop {

        print!(
"\n- Scene Structure Options -
1) go back
>> ");

        flush_output();
            
        match read_int() {
            1 => break,
            _ => print!("Wrong input\n"),
        }
    }
}

fn execute_interactive_rendering(options : &mut options::Options) {
    loop {

        print!(
"\n- Rendering Options -
1) go back
>> ");

        flush_output();
            
        match read_int() {
            1 => break,
            _ => print!("Wrong input\n"),
        }
    }
}

fn execute_interactive_performance(options : &mut options::Options) {
    loop {

        print!(
"\n- Performance Options -
1) go back
>> ");

        flush_output();
            
        match read_int() {
            1 => break,
            _ => print!("Wrong input\n"),
        }
    }
}

