use std::env;
use std::process;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let mut args = env::args().skip(1);

    let day = match args.next() {
        Some(d) => d,
        None => {
            eprintln!("Error: Program expected an argument");
            eprintln!("Note: expected something like: `cargo run [day]`");
            process::exit(1);
        }
    };

    if      day == "1" { day_1::main(); }
    else if day == "2" { day_2::main(); }
    else if day == "3" { day_3::main(); }
    else if day == "4" { day_4::main(); }
    else if day == "5" { day_5::main(); }
    else if day == "6" { day_6::main(); }
    else if day == "7" { day_7::main(); }
    else if day == "8" { day_8::main(); }
    else if day == "9" { day_9::main(); }
    else {
        println!("Error: Command not recognized");
        process::exit(1);
    }
}
