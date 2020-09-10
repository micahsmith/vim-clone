use std::io::{self};
use std::os::unix::io::RawFd;
use termios::*;

fn enable_raw_mode(fd: RawFd) -> io::Result<()> {
    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag = !(ECHO);
    return tcsetattr(fd, TCSAFLUSH, &termios);
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let fd: RawFd = 0;

    enable_raw_mode(fd).unwrap();

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

    Ok(())
}
