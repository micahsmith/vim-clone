use std::os::unix::io::RawFd;
use termios::*;

#[derive(Debug)]
pub struct Terminal {
    fd: RawFd,
    initial: Termios,
    mutable: Termios,
}

impl Terminal {
    pub fn new(stdin: RawFd) -> Terminal {
        let termios = Termios::from_fd(stdin).unwrap();

        return Terminal {
            fd: stdin,
            initial: termios,
            mutable: termios.clone(),
        };
    }

    pub fn enable_raw_mode(&self) {
        let mut termios = self.mutable;

        termios.c_cflag |= CS8;
        termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        termios.c_oflag &= !OPOST;

        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 1;

        tcsetattr(self.fd, TCSAFLUSH, &termios).unwrap();
    }

    pub fn disable_raw_mode(&self) {
        let termios = self.initial;
        tcsetattr(self.fd, TCSAFLUSH, &termios).unwrap();
    }
}
