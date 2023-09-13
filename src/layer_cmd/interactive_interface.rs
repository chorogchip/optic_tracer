
use std::io;

pub fn execute_interactive() {

    println!(
"entered interactive mode
1) input file option
2) output file option
3) scene structure option
4) rendering option
5) performance option
6) complete and execute
7) exit
>> ");


    let mut buf = String::new();
    let input_int:i64 = loop {
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let input_int:i64 = buf.trim().parse().expect("Please input an integer");
        if input_int >= 1 && input_int <= 7 {
            break input_int;
        }
    };
    print!("input: {input_int}");

    // rust is hard but a lot of fun!

}
