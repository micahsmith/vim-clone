use std::io::{self};

mod terminal;
use terminal::Terminal;

fn read_loop() {
    let mut buffer = String::new();
    loop {
        let mut input: &str = "";

        match io::stdin().read_line(&mut buffer) {
            Ok(_) => input = buffer.trim_end(),
            Err(e) => println!("{}", e),
        }

        match input {
            "q" => break,
            _ => {}
        }

        println!("{}", input);
        buffer.clear();
    }
}

fn main() -> io::Result<()> {
    let terminal = Terminal::new();
    terminal.enable_raw_mode();
    read_loop();
    terminal.disable_raw_mode();
    Ok(())
}
