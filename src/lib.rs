#[cfg(test)]
extern crate static_assertions as sa;

use std::io::{Read, Seek, Write};

pub trait Stream: Read + Write + Seek {}

impl<T> Stream for T where T: Read + Write + Seek {}

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
}
