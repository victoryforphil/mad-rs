use std::fs::File;

use log::info;
use simplelog::*;

mod sim;

fn main(){
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    info!("Hello, world!");
  
}