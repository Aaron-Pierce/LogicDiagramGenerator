extern crate image;
mod expression_parser;
mod renderer;
mod spritesheet;
use std::{
    io::{stdin},
    time::SystemTime,
};

use expression_parser::parse_boolean_expression;
use renderer::Renderer;

fn main() {

    println!("Awaiting boolean function from stdin");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    println!("{:}", buffer);

    let start_time = SystemTime::now();
    let tree = parse_boolean_expression(&buffer);
    println!(
        "Parsed boolean expression in {:?}",
        SystemTime::now().duration_since(start_time).unwrap()
    );
    let start_time = SystemTime::now();


    let mut renderer = Renderer::new(&tree);
    println!(
        "Instanced renderer in {:?}",
        SystemTime::now().duration_since(start_time).unwrap()
    );
    let start_time = SystemTime::now();
    
    renderer.draw_tree(&tree);
    renderer.export();

    println!(
        "Rendered diagram in {:?}",
        SystemTime::now().duration_since(start_time).unwrap()
    );

    let start_time = SystemTime::now();
    println!("After all that, columns are {:#?}. Took {:?}", tree.column_sizes(), SystemTime::now().duration_since(start_time).unwrap());
}
