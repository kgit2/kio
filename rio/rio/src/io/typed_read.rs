use std::io::{BufRead, Cursor, Read};

pub enum TypedRead {
    Cursor(Cursor<Vec<u8>>),
    Stdin(std::io::Stdin),
    File(std::fs::File),
    ChildStdout(std::process::ChildStdout),
    ChildStderr(std::process::ChildStderr),
    BufReader(std::io::BufReader<Box<TypedRead>>),
    LineReader(std::io::BufReader<Box<TypedRead>>),
}

impl Read for TypedRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            TypedRead::Cursor(byte_array) => byte_array.read(buf),
            TypedRead::Stdin(stdin) => stdin.read(buf),
            TypedRead::File(file) => file.read(buf),
            TypedRead::ChildStdout(child_stdout) => child_stdout.read(buf),
            TypedRead::ChildStderr(child_stderr) => child_stderr.read(buf),
            TypedRead::BufReader(buf_reader) => buf_reader.read(buf),
            TypedRead::LineReader(line_reader) => line_reader.read(buf),
        }
    }
}

impl BufRead for TypedRead {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            TypedRead::Cursor(byte_array) => byte_array.fill_buf(),
            TypedRead::Stdin(_) => panic!("BufRead not supported for stdin"),
            TypedRead::File(_) => panic!("BufRead not supported for file"),
            TypedRead::ChildStdout(_) => panic!("BufRead not supported for child stdout"),
            TypedRead::ChildStderr(_) => panic!("BufRead not supported for child stderr"),
            TypedRead::BufReader(buf_reader) => buf_reader.fill_buf(),
            TypedRead::LineReader(line_reader) => line_reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            TypedRead::Cursor(byte_array) => byte_array.consume(amt),
            TypedRead::Stdin(_) => panic!("BufRead not supported for stdin"),
            TypedRead::File(_) => panic!("BufRead not supported for file"),
            TypedRead::ChildStdout(_) => panic!("BufRead not supported for child stdout"),
            TypedRead::ChildStderr(_) => panic!("BufRead not supported for child stderr"),
            TypedRead::BufReader(buf_reader) => buf_reader.consume(amt),
            TypedRead::LineReader(line_reader) => line_reader.consume(amt),
        }
    }
}

impl TypedRead {
    pub fn as_mut_ptr(self) -> *mut std::ffi::c_void {
        Box::into_raw(Box::new(self)) as *mut std::ffi::c_void
    }

    pub fn from_mut_ptr(ptr: *mut std::ffi::c_void) -> Self {
        unsafe { *Box::from_raw(ptr as *mut TypedRead) }
    }
}

impl TypedRead {
    pub fn new_cursor() -> Self {
        TypedRead::Cursor(Cursor::new(Vec::new()))
    }

    pub fn new_stdin() -> Self {
        TypedRead::Stdin(std::io::stdin())
    }

    pub fn new_file(file: std::fs::File) -> Self {
        TypedRead::File(file)
    }

    pub fn new_child_stdout(child_stdout: std::process::ChildStdout) -> Self {
        TypedRead::ChildStdout(child_stdout)
    }

    pub fn new_child_stderr(child_stderr: std::process::ChildStderr) -> Self {
        TypedRead::ChildStderr(child_stderr)
    }

    pub fn new_buf_reader(buf_reader: std::io::BufReader<Box<TypedRead>>) -> Self {
        TypedRead::BufReader(buf_reader)
    }

    pub fn new_line_reader(line_reader: std::io::BufReader<Box<TypedRead>>) -> Self {
        TypedRead::LineReader(line_reader)
    }
}
