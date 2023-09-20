
use std::io;
use super::options;

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

    loop {

        println!(
"entered interactive mode
1) input file option
2) output file option
3) scene structure option
4) rendering option
5) performance option
6) complete and execute!
7) exit
>> ");
    
        match(read_int()) {
            7 => {
                options.add_serious_error(options::SeriousErrors::ExplicitExit(
                    String::from("exit typed on interactive mod")));
            },
            _ => (),
        }
    }

}
