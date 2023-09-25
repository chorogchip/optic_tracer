
use std::io;
use std::io::Write;
use crate::layer_option::options;
use crate::layer_option::errors;

fn read_int() -> i64 {
    let mut buf = String::new();
    loop {
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let input_int:i64 = buf.trim().parse().expect("Please input an integer");
        if input_int >= 1 && input_int <= 7 {
            break input_int;
        }
    }
}

pub fn execute_interactive(options: &mut options::Options) {

    print!("entered interactive mode\n");

    loop {

        print!(
"1) input file option
2) output file option
3) scene structure option
4) rendering option
5) performance option
6) complete and execute!
7) exit
>> ");
        
        if let Err(error) = io::stdout().flush() {
            println!("flush error:{error}");
        }
    
        match read_int() {
            7 => {
                options.errors.add_serious_error(errors::SeriousErrors::ExplicitExit(
                    String::from("exit typed on interactive mod")));
                break;
            },
            _ => (),
        }
    }

}
