use std::ffi::CString;

use std::fs;
use std::fs::File;
use std::path::*;

use std::io;
use std::io::Read;
use std::io::Write;

use libc;

pub struct FifoFile {
    path: PathBuf,
    file: File
}

impl Write for FifoFile {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }

}

impl Read for FifoFile {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }

}

pub enum FfError {
    MkfifoFailed
}

impl FifoFile {

    pub fn new(p: PathBuf) -> Result<FifoFile, FfError> {
        mkfifo(p.clone())
            .map(|_| FifoFile {
                    path: p.clone(),
                    file: File::create(p.clone()).unwrap()
                })
            .map_err(|_| FfError::MkfifoFailed)
    }

    /// The first one is the output, the second one is the input.
    pub fn new_io_pair(parent: PathBuf) -> Result<(FifoFile, FifoFile), FfError> {

        let mut pout = parent.clone();
        let mut pin = parent.clone();
        pout.push("out");
        pin.push("in");

        match (FifoFile::new(pout), FifoFile::new(pin)) {
            (Ok(fout), Ok(fin)) => Ok((fout, fin)),
            _ => Err(FfError::MkfifoFailed)
        }

    }

}

struct MkfifoError;

fn mkfifo(p: PathBuf) -> Result<(), MkfifoError> {

    let raw = CString::new(p.as_os_str().to_str().unwrap()).unwrap();
    match unsafe { libc::mkfifo(raw.as_ptr(), 0o600) } {
        0 => Ok(()),
        _ => Err(MkfifoError)
    }

}

impl Drop for FifoFile {
    fn drop(&mut self) {
        match fs::remove_file(self.path.clone()) {
            Ok(_) => {},
            Err(_) => println!("error deleting fifo: {}", self.path.to_str().unwrap())
        }
    }
}
