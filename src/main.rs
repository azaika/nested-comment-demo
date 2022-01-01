mod types;
mod lexer;
mod parser;

use anyhow::{ Result, Context };

#[macro_use]
extern crate lalrpop_util;

fn load_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).context(format!("failed to open file: {}", path))
}

fn main() -> Result<()> {
    let path = "test.txt";
    let src = load_file(path)?;
    let ast = parser::parse(&src).map_err(|err| {
        anyhow::format_err!("[{}:{}:{}] {}", path, err.span.0, err.span.1, err.item)
    })?;
    println!("{:#?}", ast);
    Ok(())
}
