use std::io::{Read, Result, Write};
use std::marker::PhantomData;

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<'a, R> {
    _phantom: PhantomData<R>,
    data: &'a [R],
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            _phantom: PhantomData,
            data: wrapped,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.data.len()
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }
}

pub struct WriteStats<W>(PhantomData<W>);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        unimplemented!()
    }

    pub fn get_ref(&self) -> &W {
        unimplemented!()
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!()
    }

    pub fn writes(&self) -> usize {
        unimplemented!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call writing {buf:?}")
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
