extern crate clap; 
use clap::{Arg, App};
use std::process::Command;
 
fn fortune() -> std::process::Output {
    return Command::new("fortune")
        .output()
        .expect("Fortune command failed to start");
}

fn execute_command(input: String, output: String) {
    Command::new(output)
    .arg(input)
    .spawn()
    .expect("Output command failed to start");
}

fn main() { 
    let matches = App::new("cowsay")
        .version("0.1")
        .about("Is a Cowsay")
        .author("Alexandre C.")
        
        .arg(Arg::from("-i --in [string] 'Enter an input String: fortune, some text...'")
            .required(true)
        )
        .arg(Arg::from("-o --out [format] 'Enter an output Format: cowsay, '")
            .required(true)
        )
        .get_matches();

    let _input = matches.value_of("in").unwrap_or("fortune").to_string();
    let _output = matches.value_of("out").unwrap_or("cowsay").to_string();

    let _input: String = match matches.value_of("in").unwrap() {
        "fortune"  => String::from_utf8_lossy(&fortune().stdout).to_string(),
        _ => _input,
    }; 

    execute_command(_input, _output);

}