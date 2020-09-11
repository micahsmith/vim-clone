use std::io::{self, Read, StdinLock};
use std::os::unix::io::AsRawFd;

mod terminal;
use terminal::Terminal;

fn is_cntrl(c: u8) -> bool {
    if c < 32 || c == 127 {
        return true;
    }
    return false;
}

fn read_loop(handle: &mut StdinLock) {
    let mut buf: [u8; 4];

    loop {
        buf = [0, 0, 0, 0];

        match handle.read(&mut buf) {
            Err(e) => println!("{}", e),
            _ => {}
        }

        match buf[0] {
            b'q' => break,
            c => {
                if !is_cntrl(c) {
                    print!("{} ({})\r\n", c, c as char)
                } else {
                    print!("{:?}\r\n", buf);
                }
            }
        }
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
