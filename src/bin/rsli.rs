use std::env;
use std::fs;

use russell::frontend::{lexer, parser};
use russell::interpreter::treewalk;

fn main() {
    // read the program
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("FATAL ERROR: No file provided.");
    let program = fs::read_to_string(filename).expect("FATAL ERROR: File cannot be read.");

    // lex the program
    let tokens = lexer::lex(&program);

    // parse the program
    let defns = parser::parse(tokens);

    // interpret the program
    treewalk::interp(defns)
}
