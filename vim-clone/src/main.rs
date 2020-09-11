use std::io::{self, Read, StdinLock};
use std::os::unix::io::AsRawFd;

mod terminal;
use terminal::Terminal;

fn read_loop(handle: &mut StdinLock) {
    let mut buf: [u8; 1] = [0; 1];

    loop {
        match handle.read(&mut buf) {
            Err(e) => println!("{}", e),
            _ => {},
        }

        match buf[0] {
            b'q' => break,
            _ => println!("{:?}", buf),
        }

        println!("{:?}", buf);
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let terminal = Terminal::new(handle.as_raw_fd());

    terminal.enable_raw_mode();
    read_loop(&mut handle);
    terminal.disable_raw_mode();

    Ok(())
}
