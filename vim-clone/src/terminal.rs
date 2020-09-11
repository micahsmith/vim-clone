use std::os::unix::io::RawFd;
use termios::*;

#[derive(Debug)]
pub struct Terminal {
    fd: RawFd,
    initial: Termios,
    mutable: Termios,
}

impl Terminal {
    pub fn new() -> Terminal {
        let fd: RawFd = 0;
        let termios = Termios::from_fd(fd).unwrap();

        return Terminal {
            fd: fd,
            initial: termios,
            mutable: termios.clone(),
        };
    }

    pub fn enable_raw_mode(&self) {
        let mut termios = self.mutable;
        termios.c_lflag &= !(ECHO);
        tcsetattr(self.fd, TCSAFLUSH, &termios).unwrap();
    }

    pub fn disable_raw_mode(&self) {
        let termios = self.initial;
        tcsetattr(self.fd, TCSAFLUSH, &termios).unwrap();
    }
}
