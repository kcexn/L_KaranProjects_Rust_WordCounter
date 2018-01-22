extern crate utils;

use std::env;

fn main() {
    let mut arguments = env::args();
    arguments.next(); 
    
    let text = match arguments.next() {
        Some(arg) => arg,
        None => {
            println!("No Arguments found!");
            return
        },
    };

    let num = utils::word_count(&text);
    println!("this string contains {} words.", num);
}
