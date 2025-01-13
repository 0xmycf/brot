use std::{env, fs};

use brot::lang;

fn vec_to_string(vec: Vec<i32>) -> String {
    let mut buf = String::with_capacity(vec.len() * 2);
    buf.push('[');
    for (i, ele) in vec.iter().enumerate() {
        buf.push_str(format!("{ele}").as_str());
        if i + 1 != vec.len() {
            buf.push_str(", ");
        }
    }
    buf.push(']');
    buf
}

/**
    Exit 0 -- all fine
    Exit 1 -- Some error occured (see stderr)
*/
fn main() {
    let file_name = env::args().nth(1).unwrap_or("./data/file.test".to_string());
    let file = fs::read_to_string(file_name).expect("Error while reading the file");

    let tokens = lang::lexer::lex(file.as_str());
    let program_ = lang::parser::parse(tokens);

    let program = match program_ {
        Err(msg) => panic!("Parse Error: {}", msg),
        Ok(v) => v,
    };

    let state = lang::interpreter::run(program);
    println!(
        "\nBrot ended in cell: {}, tape is: {}",
        state.ptr,
        vec_to_string(state.tape)
    );

    std::process::exit(0);
}

