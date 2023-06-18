use std::{env, process};

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect(); // collect 可以将迭代器转换成一个包含所有迭代器产出值的动态数组

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem paring arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
      eprintln!("Application error: {}", e);
      process::exit(1);
    }
}

