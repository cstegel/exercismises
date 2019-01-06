use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    n_reads: usize,
    bytes_read: usize,
    reader: R,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            n_reads: 0,
            bytes_read: 0,
            reader: wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.n_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.n_reads += 1;
        let result = self.reader.read(buf);
        if let Ok(n_bytes) = result {
            self.bytes_read += n_bytes;
        }
        result
    }
}

pub struct WriteStats<W> {
    n_writes: usize,
    bytes_written: usize,
    writer: W,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            n_writes: 0,
            bytes_written: 0,
            writer: wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.n_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.n_writes += 1;
        let result = self.writer.write(buf);
        if let Ok(n_bytes) = result {
            self.bytes_written += n_bytes;
        }
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
