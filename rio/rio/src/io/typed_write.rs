// use crate::result::TypedResult;
use std::io::Write;

pub enum TypedWrite {
    ByteArray(Vec<u8>),
    Stdout(std::io::Stdout),
    Stderr(std::io::Stderr),
    File(std::fs::File),
    ChildStdin(std::process::ChildStdin),
    BufWriter(std::io::BufWriter<Box<TypedWrite>>),
    LineWriter(std::io::LineWriter<Box<TypedWrite>>),
}

impl Write for TypedWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            TypedWrite::ByteArray(byte_array) => byte_array.write(buf),
            TypedWrite::Stdout(stdout) => stdout.write(buf),
            TypedWrite::Stderr(stderr) => stderr.write(buf),
            TypedWrite::File(file) => file.write(buf),
            TypedWrite::ChildStdin(child_stdin) => child_stdin.write(buf),
            TypedWrite::BufWriter(buf_writer) => buf_writer.write(buf),
            TypedWrite::LineWriter(line_writer) => line_writer.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            TypedWrite::ByteArray(_) => Ok(()),
            TypedWrite::Stdout(stdout) => stdout.flush(),
            TypedWrite::Stderr(stderr) => stderr.flush(),
            TypedWrite::File(file) => file.flush(),
            TypedWrite::ChildStdin(child_stdin) => child_stdin.flush(),
            TypedWrite::BufWriter(buf_writer) => buf_writer.flush(),
            TypedWrite::LineWriter(line_writer) => line_writer.flush(),
        }
    }
}

impl TypedWrite {
    pub fn as_mut_ptr(self) -> *mut std::ffi::c_void {
        Box::into_raw(Box::new(self)) as *mut std::ffi::c_void
    }

    pub fn from_mut_ptr(ptr: *mut std::ffi::c_void) -> Self {
        unsafe { *Box::from_raw(ptr as *mut TypedWrite) }
    }
}

impl TypedWrite {
    pub fn new_byte_array() -> Self {
        TypedWrite::ByteArray(Vec::new())
    }

    pub fn new_stdout() -> Self {
        TypedWrite::Stdout(std::io::stdout())
    }

    pub fn new_stderr() -> Self {
        TypedWrite::Stderr(std::io::stderr())
    }

    pub fn new_file(file: std::fs::File) -> Self {
        TypedWrite::File(file)
    }

    pub fn new_child_stdin(child_stdin: std::process::ChildStdin) -> Self {
        TypedWrite::ChildStdin(child_stdin)
    }

    pub fn new_buf_writer(inner: TypedWrite) -> Self {
        TypedWrite::BufWriter(std::io::BufWriter::new(Box::new(inner)))
    }

    pub fn new_line_writer(inner: TypedWrite) -> Self {
        TypedWrite::LineWriter(std::io::LineWriter::new(Box::new(inner)))
    }
}

#[no_mangle]
pub extern "C" fn create_stdout_ptr() -> usize {
    println!("new_stdout_ptr");
    64
}
