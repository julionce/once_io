#[cfg(test)]
extern crate static_assertions as sa;

use std::cmp::min;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

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

impl<T> Chunk<T> {
    fn remainder_len(&self) -> u64 {
        match self.limit {
            None => u64::MAX,
            Some(v) => match v {
                u64::MAX => u64::MAX,
                _ => match v.checked_sub(self.pos) {
                    None => 0u64,
                    Some(v) => v,
                },
            },
        }
    }
}

impl<T> Chunk<T>
where
    T: Stream,
{
    pub fn into_inner(self) -> T {
        self.inner
    }

    fn start_position(&mut self) -> Result<u64> {
        Ok(self.inner.stream_position()? - self.pos)
    }

    fn end_position(&mut self) -> Result<u64> {
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

impl<T> Seek for Chunk<T>
where
    T: Stream,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let start_position = self.start_position()?;
        let end_position = self.end_position()?;
        let final_position = match pos {
            SeekFrom::Current(v) => self.inner.stream_position()?.checked_add_signed(v),
            SeekFrom::End(v) => end_position.checked_add_signed(v),
            SeekFrom::Start(v) => start_position.checked_add(v),
        };

        match final_position {
            Some(v) if v >= start_position && v <= end_position => {
                self.pos = self.inner.seek(SeekFrom::Start(v))? - start_position;
                Ok(self.pos)
            }
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.pos)
    }
}

impl<T> Read for Chunk<T>
where
    T: Stream,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let max = min(buf.len() as u64, self.remainder_len()) as usize;
        let n = self.inner.read(&mut buf[..max])?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T> Write for Chunk<T>
where
    T: Stream,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let max = min(buf.len() as u64, self.remainder_len()) as usize;
        let n = self.inner.write(&buf[..max])?;
        self.pos += n as u64;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
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

    #[test]
    fn assert_seek_from_start_impl_for_chunk() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.seek(SeekFrom::Start(0)).unwrap(), 0u64);
        assert_eq!(chunk.seek(SeekFrom::Start(1)).unwrap(), 1u64);
    }

    #[test]
    fn assert_seek_from_current_impl_for_chunk() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.seek(SeekFrom::Current(0)).unwrap(), 0u64);
        assert_eq!(chunk.seek(SeekFrom::Current(1)).unwrap(), 1u64);
        assert_eq!(chunk.seek(SeekFrom::Current(-1)).unwrap(), 0u64);
    }

    #[test]
    fn assert_seek_from_end_impl_for_chunk() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.seek(SeekFrom::End(0)).unwrap(), data.len() as u64);
        assert_eq!(
            chunk.seek(SeekFrom::End(-1)).unwrap(),
            (data.len() - 1) as u64
        );
    }

    #[test]
    fn assert_seek_beyond_end_impl_for_chunk_with_limit() {
        let limit = 5u64;
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(Some(limit));
        assert_eq!(chunk.seek(SeekFrom::Start(limit)).unwrap(), limit);
        assert!(chunk.seek(SeekFrom::Start(limit + 1)).is_err());
        assert_eq!(chunk.stream_position().unwrap(), limit);
    }

    #[test]
    fn assert_seek_beyond_end_impl_for_chunk_with_limit_beyond_end() {
        let data = [0u8; 10];
        let limit = data.len() as u64 + 1;
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(Some(limit));
        let len = data.len() as u64;
        assert_eq!(chunk.seek(SeekFrom::Start(len)).unwrap(), len);
        assert!(chunk.seek(SeekFrom::Start(len + 1)).is_err());
        assert_eq!(chunk.stream_position().unwrap(), len);
    }

    #[test]
    fn assert_seek_beyond_end_impl_for_chunk_without_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        let len = data.len() as u64;
        assert_eq!(chunk.seek(SeekFrom::Start(len)).unwrap(), len);
        assert!(chunk.seek(SeekFrom::Start(len + 1)).is_err());
        assert_eq!(chunk.stream_position().unwrap(), len);
    }

    #[test]
    fn assert_seek_negative_impl_for_chunk() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert!(chunk.seek(SeekFrom::Current(-1)).is_err());
        assert_eq!(chunk.stream_position().unwrap(), 0u64);
    }

    #[test]
    fn assert_stream_position_impl_for_chunk() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        assert_eq!(chunk.seek(SeekFrom::Start(0)).unwrap(), 0u64);
        assert_eq!(chunk.stream_position().unwrap(), 0u64);
        assert_eq!(chunk.seek(SeekFrom::Start(1)).unwrap(), 1u64);
        assert_eq!(chunk.stream_position().unwrap(), 1u64);
    }

    #[test]
    fn assert_into_inner_impl_for_chunk() {
        let data = [0u8; 10];
        let mut stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        chunk.seek(SeekFrom::Start(1)).unwrap();
        stream = chunk.into_inner();
        assert_eq!(stream.stream_position().unwrap(), 1u64);
    }

    #[test]
    fn assert_remainder_length_impl_for_chunk_with_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let limit = 5u64;
        let chunk = stream.chunk(Some(limit));
        assert_eq!(chunk.remainder_len(), limit);
    }

    #[test]
    fn assert_remainder_length_impl_for_chunk_with_max_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let limit = u64::MAX;
        let chunk = stream.chunk(Some(limit));
        assert_eq!(chunk.remainder_len(), u64::MAX);
    }

    #[test]
    fn assert_remainder_length_impl_for_chunk_without_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let chunk = stream.chunk(None);
        assert_eq!(chunk.remainder_len(), u64::MAX);
    }

    #[test]
    fn assert_read_impl_for_chunk_with_limit() {
        let data = [0u8; 10];
        let stream = Cursor::new(data);
        let limit = 5u64;
        let mut chunk = stream.chunk(Some(limit));
        let original = [1u8; 10];
        let mut buf = original;
        assert_eq!(chunk.read(&mut buf).unwrap(), limit as usize);
        assert_eq!(buf[..(limit as usize)], data[..(limit as usize)]);
        assert_eq!(buf[(limit as usize)..], original[(limit as usize)..]);
        assert_eq!(chunk.stream_position().unwrap(), limit);
    }

    #[test]
    fn assert_read_impl_for_chunk_without_limit() {
        let data = [0u8; 10];
        let stream_len = data.len();
        let stream = Cursor::new(data);
        let mut chunk = stream.chunk(None);
        let original = [1u8; 11];
        let mut buf = original;
        assert_eq!(chunk.read(&mut buf).unwrap(), stream_len);
        assert_eq!(buf[..stream_len], data);
        assert_eq!(buf[stream_len..], original[stream_len..]);
        assert_eq!(chunk.stream_position().unwrap(), stream_len as u64);
    }

    #[test]
    fn assert_write_impl_for_chunk_with_limit() {
        let original = [0u8; 10];
        let mut data = original;
        let stream = Cursor::new(data.as_mut());
        let limit = 5u64;
        let mut chunk = stream.chunk(Some(limit));
        let buf = [1u8; 10];
        assert_eq!(chunk.write(&buf).unwrap(), limit as usize);
        assert_eq!(chunk.stream_position().unwrap(), limit);
        assert_eq!(buf[..(limit as usize)], data[..(limit as usize)]);
        assert_eq!(data[(limit as usize)..], original[(limit as usize)..]);
    }

    #[test]
    fn assert_write_impl_for_chunk_without_limit() {
        let original = [0u8; 10];
        let mut data = original;
        let stream_len = data.len();
        let stream = Cursor::new(data.as_mut());
        let mut chunk = stream.chunk(None);
        let buf = [1u8; 11];
        assert_eq!(chunk.write(&buf).unwrap(), stream_len);
        assert_eq!(chunk.stream_position().unwrap(), stream_len as u64);
        assert_eq!(buf[..stream_len], data);
    }
}
