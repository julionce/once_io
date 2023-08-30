#[cfg(test)]
extern crate static_assertions as sa;

use std::cmp::min;
use std::io::{Read, Seek, SeekFrom, Write};

pub trait Stream: Read + Write + Seek {
    fn chunk(self, limit: Option<u64>) -> Chunk<Self>
    where
        Self: Sized,
    {
        Chunk {
            inner: self,
            pos: 0,
            limit,
        }
    }
}

impl<T> Stream for T where T: Read + Write + Seek {}

pub struct Chunk<T> {
    inner: T,
    pos: u64,
    limit: Option<u64>,
}

impl<T> Chunk<T>
where
    T: Stream,
{
    fn start_position(&mut self) -> std::io::Result<u64> {
        Ok(self.inner.stream_position()? - self.pos)
    }

    fn end_position(&mut self) -> std::io::Result<u64> {
        let rollback_position = self.inner.stream_position()?;
        let end_position = match self.limit {
            None => self.inner.seek(SeekFrom::End(0))?,
            Some(v) => match self.start_position()?.checked_add(v) {
                None => self.inner.seek(SeekFrom::End(0))?,
                Some(v) => min(v, self.inner.seek(SeekFrom::End(0))?),
            },
        };
        self.inner.seek(SeekFrom::Start(rollback_position))?;
        Ok(end_position)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use sa::assert_impl_all;

    use super::*;

    #[test]
    fn assert_stream_impl() {
        assert_impl_all!(dyn Stream: Read, Write, Seek);
        assert_impl_all!(Cursor<Vec<u8>>: Read, Write, Seek, Stream);
    }

    #[test]
    fn assert_start_position_with_no_offset() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.start_position().unwrap(), 0);
    }

    #[test]
    fn assert_start_position_with_offset() {
        let data = [0u8; 10];
        let mut stream = Cursor::new(data);
        stream.seek(SeekFrom::Start(1)).unwrap();
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.start_position().unwrap(), 1);
    }

    #[test]
    fn assert_end_position_with_no_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.end_position().unwrap(), data.len() as u64);
    }

    #[test]
    fn assert_end_position_with_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(Some(5u64));
        assert_eq!(chunk.end_position().unwrap(), 5u64);
    }

    #[test]
    fn assert_end_position_with_limit_and_offset() {
        let data = [0u8; 10];
        let mut stream = Cursor::new(data);
        stream.seek(SeekFrom::Start(1)).unwrap();
        let mut chunk = stream.chunk(Some(5u64));
        assert_eq!(chunk.end_position().unwrap(), 6u64);
    }

    #[test]
    fn assert_end_position_with_limit_behind_end() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(Some(11u64));
        assert_eq!(chunk.end_position().unwrap(), data.len() as u64);
    }
}
