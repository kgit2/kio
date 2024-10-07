use std::io::{Cursor, Write};

pub enum RioWrite {
    ByteArray(Cursor<&'static mut [u8]>),
    Stdout(std::io::Stdout),
    Stderr(std::io::Stderr),
    File(std::fs::File),
    ChildStdin(std::process::ChildStdin),
    BufWriter(std::io::BufWriter<Box<RioWrite>>),
    LineWriter(std::io::LineWriter<Box<RioWrite>>),
}

impl Write for RioWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            RioWrite::ByteArray(byte_array) => byte_array.write(buf),
            RioWrite::Stdout(stdout) => stdout.write(buf),
            RioWrite::Stderr(stderr) => stderr.write(buf),
            RioWrite::File(file) => file.write(buf),
            RioWrite::ChildStdin(child_stdin) => child_stdin.write(buf),
            RioWrite::BufWriter(buf_writer) => buf_writer.write(buf),
            RioWrite::LineWriter(line_writer) => line_writer.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            RioWrite::ByteArray(_) => Ok(()),
            RioWrite::Stdout(stdout) => stdout.flush(),
            RioWrite::Stderr(stderr) => stderr.flush(),
            RioWrite::File(file) => file.flush(),
            RioWrite::ChildStdin(child_stdin) => child_stdin.flush(),
            RioWrite::BufWriter(buf_writer) => buf_writer.flush(),
            RioWrite::LineWriter(line_writer) => line_writer.flush(),
        }
    }
}

impl RioWrite {
    pub fn as_mut_ptr(self) -> *mut std::ffi::c_void {
        Box::into_raw(Box::new(self)) as *mut std::ffi::c_void
    }

    pub fn from_mut_ptr(ptr: *mut std::ffi::c_void) -> Self {
        unsafe { *Box::from_raw(ptr as *mut RioWrite) }
    }
}

impl RioWrite {
    pub fn new_byte_array(byte_array: &'static mut [u8]) -> Self {
        RioWrite::ByteArray(Cursor::new(byte_array))
    }

    pub fn new_stdout() -> Self {
        RioWrite::Stdout(std::io::stdout())
    }

    pub fn new_stderr() -> Self {
        RioWrite::Stderr(std::io::stderr())
    }

    pub fn new_file(file: std::fs::File) -> Self {
        RioWrite::File(file)
    }

    pub fn new_child_stdin(child_stdin: std::process::ChildStdin) -> Self {
        RioWrite::ChildStdin(child_stdin)
    }

    pub fn new_buf_writer(inner: Box<RioWrite>) -> Self {
        RioWrite::BufWriter(std::io::BufWriter::new(inner))
    }

    pub fn new_line_writer(inner: Box<RioWrite>) -> Self {
        RioWrite::LineWriter(std::io::LineWriter::new(inner))
    }
}
