#![feature(seek_stream_len)]
pub mod read_num;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

pub struct Stream<'a, T>
where
    T: Seek,
{
    inner: &'a mut T,
    origin_pos: u64,
    limit_pos: u64,
}

impl<'a, T> Stream<'a, T>
where
    T: Seek,
{
    pub fn new(inner: &'a mut T) -> Stream<'a, T> {
        Stream::<'a, T> {
            inner,
            origin_pos: 0,
            limit_pos: u64::MAX,
        }
    }
}

impl<'a, T> Stream<'a, T>
where
    T: Seek,
{
    pub fn borrow_chunk(&mut self, limit: Option<u64>) -> Result<Stream<'_, T>> {
        let origin_pos = self.inner.stream_position()?;
        let limit_pos = match limit {
            None => u64::MAX,
            Some(l) => std::cmp::min(origin_pos.saturating_add(l), self.limit_pos),
        };
        Ok(Stream::<'_, T> {
            inner: self.inner,
            origin_pos,
            limit_pos,
        })
    }

    pub fn remainder_len(&mut self) -> Result<u64> {
        let current_position = self.inner.stream_position()?;
        let end_position = std::cmp::min(self.inner.stream_len()?, self.limit_pos);
        Ok(match end_position.checked_sub(current_position) {
            Some(n) => n,
            None => 0,
        })
    }
}

impl<T> Seek for Stream<'_, T>
where
    T: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let start_position = self.origin_pos;
        let end_position = std::cmp::min(self.inner.stream_len()?, self.limit_pos);
        let final_position = match pos {
            SeekFrom::Current(n) => self.inner.stream_position()?.checked_add_signed(n),
            SeekFrom::End(n) => end_position.checked_add_signed(n),
            SeekFrom::Start(n) => start_position.checked_add(n),
        };
        let relative_position = match final_position {
            Some(n) => n.checked_sub(self.origin_pos),
            None => None,
        };
        match (final_position, relative_position) {
            (Some(f), Some(r)) if f <= end_position => {
                self.inner.seek(SeekFrom::Start(f))?;
                Ok(r)
            }
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

impl<T> Read for Stream<'_, T>
where
    T: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = std::cmp::min(buf.len(), self.remainder_len()? as usize);
        Ok(self.inner.read(&mut buf[..len])?)
    }
}

impl<T> Write for Stream<'_, T>
where
    T: Write + Seek,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = std::cmp::min(buf.len(), self.remainder_len()? as usize);
        Ok(self.inner.write(&buf[..len])?)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn new_stream() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let stream = Stream::new(&mut cursor);
        assert_eq!(stream.origin_pos, 0);
        assert_eq!(stream.limit_pos, u64::MAX);
        assert!(std::ptr::eq(stream.inner, &cursor));
    }

    #[test]
    fn borrow_chunk_without_offset() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let chunk = stream.borrow_chunk(None).unwrap();
        assert_eq!(chunk.origin_pos, 0);
    }

    #[test]
    fn borrow_chunk_with_offset() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let _ = stream.seek(SeekFrom::Start(1));
        let chunk = stream.borrow_chunk(None).unwrap();
        assert_eq!(chunk.origin_pos, 1);
    }

    #[test]
    fn borrow_chunk_without_limit() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let chunk = stream.borrow_chunk(None).unwrap();
        assert_eq!(chunk.limit_pos, u64::MAX);
    }

    #[test]
    fn borrow_chunk_with_under_stream_limit() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let chunk = stream.borrow_chunk(Some(11)).unwrap();
        assert_eq!(chunk.limit_pos, 11);
    }

    #[test]
    fn borrow_chunk_with_over_stream_limit() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream_foo = Stream::new(&mut cursor);
        let mut stream_bar = stream_foo.borrow_chunk(Some(10)).unwrap();
        let chunk = stream_bar.borrow_chunk(Some(11)).unwrap();
        assert_eq!(chunk.limit_pos, 10);
    }

    #[test]
    fn remainder_len_with_limit_over_stream_len() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        assert_eq!(stream.remainder_len().unwrap(), 10);
        let _ = stream.seek(SeekFrom::Current(1));
        assert_eq!(stream.remainder_len().unwrap(), 9);
    }

    #[test]
    fn remainder_len_with_limit_under_stream_len() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        assert_eq!(chunk.remainder_len().unwrap(), 9);
    }

    #[test]
    fn remainder_len_with_stream_position_over_limit() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let _ = chunk.inner.seek(SeekFrom::End(0));
        assert_eq!(chunk.remainder_len().unwrap(), 0);
    }

    /*
    TODO: implement tests to check the following scenarios.
    | offset        | limit                   | seek_from         | seek_to                  |
    | with/without  | under/at/over inner end | current/start/end | under/at/over stream end |
    */

    #[test]
    fn seek_from_current() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::Current(10));
        assert_eq!(seek_result.ok(), Some(10));
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::Current(9));
        assert_eq!(seek_result.ok(), Some(9));
    }

    #[test]
    fn seek_from_current_when_overflow() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::Current(11));
        assert!(seek_result.is_err());
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::Current(10));
        assert!(seek_result.is_err());
    }

    #[test]
    fn seek_from_current_when_underflow() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::Current(-1));
        assert!(seek_result.is_err());
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::Current(-1));
        assert!(seek_result.is_err());
    }

    #[test]
    fn seek_from_end() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::End(-10));
        assert_eq!(seek_result.ok(), Some(0));
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::End(-9));
        assert_eq!(seek_result.ok(), Some(0));
    }

    #[test]
    fn seek_from_end_when_overflow() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::End(-11));
        assert!(seek_result.is_err());
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::End(-10));
        assert!(seek_result.is_err());
    }

    #[test]
    fn seek_from_end_when_underflow() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::End(1));
        assert!(seek_result.is_err());
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::End(1));
        assert!(seek_result.is_err());
    }

    #[test]
    fn seek_from_start() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::Start(10));
        assert_eq!(seek_result.ok(), Some(10));
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(9)).unwrap();
        let seek_result = chunk.seek(SeekFrom::Start(9));
        assert_eq!(seek_result.ok(), Some(9));
    }

    #[test]
    fn seek_from_start_when_overflow() {
        let data = [0u8; 10];
        let mut cursor = Cursor::new(data);
        let mut stream = Stream::new(&mut cursor);
        let seek_result = stream.seek(SeekFrom::Start(11));
        assert!(seek_result.is_err());
        let _ = stream.seek(SeekFrom::Start(1));
        let mut chunk = stream.borrow_chunk(Some(10)).unwrap();
        let seek_result = chunk.seek(SeekFrom::Start(10));
        assert!(seek_result.is_err());
    }
}
