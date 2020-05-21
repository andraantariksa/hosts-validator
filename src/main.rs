extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::process::exit;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HostsParser;

fn main() {
    let filename = match std::env::args().nth(1) {
        Some(filename) => filename,
        None => {
            eprintln!("Please specify the hosts file path");
            exit(-1);
        }
    };

    let str = match std::fs::read_to_string(filename) {
        Ok(str) => str,
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    };

    let return_code = match HostsParser::parse(Rule::main, &str) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            -1
        }
    };
    exit(return_code);
}
