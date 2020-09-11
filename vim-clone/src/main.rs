use std::io::{self};
use std::os::unix::io::RawFd;
use termios::*;

#[derive(Debug)]
struct Terminal {
    fd: RawFd,
    initial: Termios,
    mutable: Termios,
}

impl Terminal {
    fn new() -> Terminal {
        let fd: RawFd = 0;
        let termios = Termios::from_fd(fd).unwrap();

        return Terminal {
            fd: fd,
            initial: termios,
            mutable: termios.clone(),
        };
    }

    fn enable_raw_mode(&mut self) {
        let mut termios = self.mutable;
        termios.c_lflag &= !(ECHO);
        tcsetattr(self.fd, TCSAFLUSH, &termios).unwrap();
    }

    fn disable_raw_mode(&mut self) {
        let termios = self.initial;
        tcsetattr(self.fd, TCSAFLUSH, &termios).unwrap();
    }
}

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new();
    terminal.enable_raw_mode();

    let mut buffer = String::new();
    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if buffer == "q\n" {
                    break;
                }
                println!("{} bytes read", n);
                println!("{}", buffer);
            }
            Err(e) => println!("{}", e),
        }

        buffer.clear();
    }

    terminal.disable_raw_mode();
    Ok(())
}