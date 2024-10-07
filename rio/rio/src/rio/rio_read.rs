use std::io::{BufRead, Cursor, Read};

pub enum RioRead {
    ByteArray(Cursor<&'static mut [u8]>),
    Stdin(std::io::Stdin),
    File(std::fs::File),
    ChildStdout(std::process::ChildStdout),
    ChildStderr(std::process::ChildStderr),
    BufReader(std::io::BufReader<Box<RioRead>>),
}

impl Read for RioRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            RioRead::ByteArray(byte_array) => byte_array.read(buf),
            RioRead::Stdin(stdin) => stdin.read(buf),
            RioRead::File(file) => file.read(buf),
            RioRead::ChildStdout(child_stdout) => child_stdout.read(buf),
            RioRead::ChildStderr(child_stderr) => child_stderr.read(buf),
            RioRead::BufReader(buf_reader) => buf_reader.read(buf),
        }
    }
}

impl BufRead for RioRead {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            RioRead::ByteArray(byte_array) => byte_array.fill_buf(),
            RioRead::Stdin(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BufRead not supported for stdin",
            )),
            RioRead::File(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BufRead not supported for file",
            )),
            RioRead::ChildStdout(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BufRead not supported for child stdout",
            )),
            RioRead::ChildStderr(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BufRead not supported for child stderr",
            )),
            RioRead::BufReader(buf_reader) => buf_reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            RioRead::ByteArray(byte_array) => byte_array.consume(amt),
            RioRead::Stdin(_) => panic!("BufRead not supported for stdin"),
            RioRead::File(_) => panic!("BufRead not supported for file"),
            RioRead::ChildStdout(_) => panic!("BufRead not supported for child stdout"),
            RioRead::ChildStderr(_) => panic!("BufRead not supported for child stderr"),
            RioRead::BufReader(buf_reader) => buf_reader.consume(amt),
        }
    }
}

impl RioRead {
    pub fn as_mut_ptr(self) -> *mut std::ffi::c_void {
        Box::into_raw(Box::new(self)) as *mut std::ffi::c_void
    }

    pub fn from_mut_ptr(ptr: *mut std::ffi::c_void) -> Self {
        unsafe { *Box::from_raw(ptr as *mut RioRead) }
    }
}

impl RioRead {
    pub fn new_byte_array(byte_array: &'static mut [u8]) -> Self {
        RioRead::ByteArray(Cursor::new(byte_array))
    }

    pub fn new_stdin() -> Self {
        RioRead::Stdin(std::io::stdin())
    }

    pub fn new_file(file: std::fs::File) -> Self {
        RioRead::File(file)
    }

    pub fn new_child_stdout(child_stdout: std::process::ChildStdout) -> Self {
        RioRead::ChildStdout(child_stdout)
    }

    pub fn new_child_stderr(child_stderr: std::process::ChildStderr) -> Self {
        RioRead::ChildStderr(child_stderr)
    }

    pub fn new_buf_reader(buf_reader: Box<RioRead>) -> Self {
        RioRead::BufReader(std::io::BufReader::new(buf_reader))
    }
}
