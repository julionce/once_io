use std::{
    io::{Read, Result},
    mem,
};

pub trait NumReader<T: ?Sized> {
    fn read_u8(_: &mut T) -> Result<u8>;
    fn read_u16(_: &mut T) -> Result<u16>;
    fn read_u32(_: &mut T) -> Result<u32>;
    fn read_u64(_: &mut T) -> Result<u64>;
    fn read_u128(_: &mut T) -> Result<u128>;

    fn read_i8(_: &mut T) -> Result<i8>;
    fn read_i16(_: &mut T) -> Result<i16>;
    fn read_i32(_: &mut T) -> Result<i32>;
    fn read_i64(_: &mut T) -> Result<i64>;
    fn read_i128(_: &mut T) -> Result<i128>;

    fn read_usize(_: &mut T) -> Result<usize>;
    fn read_isize(_: &mut T) -> Result<isize>;

    fn read_f32(_: &mut T) -> Result<f32>;
    fn read_f64(_: &mut T) -> Result<f64>;
}

pub trait ReadNum {
    type Reader: NumReader<Self>;

    fn read_u8(&mut self) -> Result<u8> {
        Self::Reader::read_u8(self)
    }

    fn read_u16(&mut self) -> Result<u16> {
        Self::Reader::read_u16(self)
    }

    fn read_u32(&mut self) -> Result<u32> {
        Self::Reader::read_u32(self)
    }

    fn read_u64(&mut self) -> Result<u64> {
        Self::Reader::read_u64(self)
    }

    fn read_u128(&mut self) -> Result<u128> {
        Self::Reader::read_u128(self)
    }

    fn read_i8(&mut self) -> Result<i8> {
        Self::Reader::read_i8(self)
    }

    fn read_i16(&mut self) -> Result<i16> {
        Self::Reader::read_i16(self)
    }

    fn read_i32(&mut self) -> Result<i32> {
        Self::Reader::read_i32(self)
    }

    fn read_i64(&mut self) -> Result<i64> {
        Self::Reader::read_i64(self)
    }

    fn read_i128(&mut self) -> Result<i128> {
        Self::Reader::read_i128(self)
    }

    fn read_usize(&mut self) -> Result<usize> {
        Self::Reader::read_usize(self)
    }

    fn read_isize(&mut self) -> Result<isize> {
        Self::Reader::read_isize(self)
    }

    fn read_f32(&mut self) -> Result<f32> {
        Self::Reader::read_f32(self)
    }

    fn read_f64(&mut self) -> Result<f64> {
        Self::Reader::read_f64(self)
    }
}

macro_rules! impl_num_reader_be {
    ($type: ty, $method: ident) => {
        fn $method(reader: &mut T) -> Result<$type> {
            let mut buf = [0u8; mem::size_of::<$type>()];
            match reader.read_exact(&mut buf) {
                Ok(()) => Ok(<$type>::from_be_bytes(buf)),
                Err(e) => Err(e),
            }
        }
    };
}

pub struct BigEndianReader;

impl<T> NumReader<T> for BigEndianReader
where
    T: Read,
{
    impl_num_reader_be! {u8, read_u8}
    impl_num_reader_be! {u16, read_u16}
    impl_num_reader_be! {u32, read_u32}
    impl_num_reader_be! {u64, read_u64}
    impl_num_reader_be! {u128, read_u128}
    impl_num_reader_be! {i8, read_i8}
    impl_num_reader_be! {i16, read_i16}
    impl_num_reader_be! {i32, read_i32}
    impl_num_reader_be! {i64, read_i64}
    impl_num_reader_be! {i128, read_i128}
    impl_num_reader_be! {usize, read_usize}
    impl_num_reader_be! {isize, read_isize}
    impl_num_reader_be! {f32, read_f32}
    impl_num_reader_be! {f64, read_f64}
}

macro_rules! impl_num_reader_le {
    ($type: ty, $method: ident) => {
        fn $method(reader: &mut T) -> Result<$type> {
            let mut buf = [0u8; mem::size_of::<$type>()];
            match reader.read_exact(&mut buf) {
                Ok(()) => Ok(<$type>::from_le_bytes(buf)),
                Err(e) => Err(e),
            }
        }
    };
}

pub struct LittleEndianReader;

impl<T> NumReader<T> for LittleEndianReader
where
    T: Read,
{
    impl_num_reader_le! {u8, read_u8}
    impl_num_reader_le! {u16, read_u16}
    impl_num_reader_le! {u32, read_u32}
    impl_num_reader_le! {u64, read_u64}
    impl_num_reader_le! {u128, read_u128}
    impl_num_reader_le! {i8, read_i8}
    impl_num_reader_le! {i16, read_i16}
    impl_num_reader_le! {i32, read_i32}
    impl_num_reader_le! {i64, read_i64}
    impl_num_reader_le! {i128, read_i128}
    impl_num_reader_le! {usize, read_usize}
    impl_num_reader_le! {isize, read_isize}
    impl_num_reader_le! {f32, read_f32}
    impl_num_reader_le! {f64, read_f64}
}

macro_rules! impl_num_reader_ne {
    ($type: ty, $method: ident) => {
        fn $method(reader: &mut T) -> Result<$type> {
            let mut buf = [0u8; mem::size_of::<$type>()];
            match reader.read_exact(&mut buf) {
                Ok(()) => Ok(<$type>::from_le_bytes(buf)),
                Err(e) => Err(e),
            }
        }
    };
}

pub struct NativeEndianReader;

impl<T> NumReader<T> for NativeEndianReader
where
    T: Read,
{
    impl_num_reader_ne! {u8, read_u8}
    impl_num_reader_ne! {u16, read_u16}
    impl_num_reader_ne! {u32, read_u32}
    impl_num_reader_ne! {u64, read_u64}
    impl_num_reader_ne! {u128, read_u128}
    impl_num_reader_ne! {i8, read_i8}
    impl_num_reader_ne! {i16, read_i16}
    impl_num_reader_ne! {i32, read_i32}
    impl_num_reader_ne! {i64, read_i64}
    impl_num_reader_ne! {i128, read_i128}
    impl_num_reader_ne! {usize, read_usize}
    impl_num_reader_ne! {isize, read_isize}
    impl_num_reader_ne! {f32, read_f32}
    impl_num_reader_ne! {f64, read_f64}
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    struct BEReader<T> {
        inner: T,
    }

    impl<T> Read for BEReader<T>
    where
        T: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.inner.read(buf)
        }
    }

    impl<T> ReadNum for BEReader<T>
    where
        T: Read,
    {
        type Reader = BigEndianReader;
    }

    macro_rules! generate_read_num_be_test {
        ($test_name: ident, $type: ty, $value: expr, $method: ident) => {
            #[test]
            fn $test_name() {
                let data = $value.to_be_bytes();
                let mut reader = BEReader {
                    inner: Cursor::new(data),
                };
                assert_eq!(reader.$method().unwrap(), $value);
            }
        };
    }

    generate_read_num_be_test! {read_num_u8_val_be, u8, 11u8, read_u8}
    generate_read_num_be_test! {read_num_u8_max_be, u8, u8::MAX, read_u8}
    generate_read_num_be_test! {read_num_u8_min_be, u8, u8::MIN, read_u8}
    generate_read_num_be_test! {read_num_u16_val_be, u16, 11u16, read_u16}
    generate_read_num_be_test! {read_num_u16_max_be, u16, u16::MAX, read_u16}
    generate_read_num_be_test! {read_num_u16_min_be, u16, u16::MIN, read_u16}
    generate_read_num_be_test! {read_num_u32_val_be, u32, 11u32, read_u32}
    generate_read_num_be_test! {read_num_u32_max_be, u32, u32::MAX, read_u32}
    generate_read_num_be_test! {read_num_u32_min_be, u32, u32::MIN, read_u32}
    generate_read_num_be_test! {read_num_u64_val_be, u64, 11u64, read_u64}
    generate_read_num_be_test! {read_num_u64_max_be, u64, u64::MAX, read_u64}
    generate_read_num_be_test! {read_num_u64_min_be, u64, u64::MIN, read_u64}
    generate_read_num_be_test! {read_num_u128_val_be, u128, 11u128, read_u128}
    generate_read_num_be_test! {read_num_u128_max_be, u128, u128::MAX, read_u128}
    generate_read_num_be_test! {read_num_u128_min_be, u128, u128::MIN, read_u128}
    generate_read_num_be_test! {read_num_i8_val_be, i8, 11i8, read_i8}
    generate_read_num_be_test! {read_num_i8_max_be, i8, i8::MAX, read_i8}
    generate_read_num_be_test! {read_num_i8_min_be, i8, i8::MIN, read_i8}
    generate_read_num_be_test! {read_num_i16_val_be, i16, 11i16, read_i16}
    generate_read_num_be_test! {read_num_i16_max_be, i16, i16::MAX, read_i16}
    generate_read_num_be_test! {read_num_i16_min_be, i16, i16::MIN, read_i16}
    generate_read_num_be_test! {read_num_i32_val_be, i32, 11i32, read_i32}
    generate_read_num_be_test! {read_num_i32_max_be, i32, i32::MAX, read_i32}
    generate_read_num_be_test! {read_num_i32_min_be, i32, i32::MIN, read_i32}
    generate_read_num_be_test! {read_num_i64_val_be, i64, 11i64, read_i64}
    generate_read_num_be_test! {read_num_i64_max_be, i64, i64::MAX, read_i64}
    generate_read_num_be_test! {read_num_i64_min_be, i64, i64::MIN, read_i64}
    generate_read_num_be_test! {read_num_i128_val_be, i128, 11i128, read_i128}
    generate_read_num_be_test! {read_num_i128_max_be, i128, i128::MAX, read_i128}
    generate_read_num_be_test! {read_num_i128_min_be, i128, i128::MIN, read_i128}
    generate_read_num_be_test! {read_num_usize_val_be, usize, 11usize, read_usize}
    generate_read_num_be_test! {read_num_usize_max_be, usize, usize::MAX, read_usize}
    generate_read_num_be_test! {read_num_usize_min_be, usize, usize::MIN, read_usize}
    generate_read_num_be_test! {read_num_isize_val_be, isize, 11isize, read_isize}
    generate_read_num_be_test! {read_num_isize_max_be, isize, isize::MAX, read_isize}
    generate_read_num_be_test! {read_num_isize_min_be, isize, isize::MIN, read_isize}
    generate_read_num_be_test! {read_num_f32_val_be, f32, 11f32, read_f32}
    generate_read_num_be_test! {read_num_f32_max_be, f32, f32::MAX, read_f32}
    generate_read_num_be_test! {read_num_f32_min_be, f32, f32::MIN, read_f32}
    generate_read_num_be_test! {read_num_f64_val_be, f64, 11f64, read_f64}
    generate_read_num_be_test! {read_num_f64_max_be, f64, f64::MAX, read_f64}
    generate_read_num_be_test! {read_num_f64_min_be, f64, f64::MIN, read_f64}

    struct LEReader<T> {
        inner: T,
    }

    impl<T> Read for LEReader<T>
    where
        T: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.inner.read(buf)
        }
    }

    impl<T> ReadNum for LEReader<T>
    where
        T: Read,
    {
        type Reader = LittleEndianReader;
    }

    macro_rules! generate_read_num_le_test {
        ($test_name: ident, $type: ty, $value: expr, $method: ident) => {
            #[test]
            fn $test_name() {
                let data = $value.to_le_bytes();
                let mut reader = LEReader {
                    inner: Cursor::new(data),
                };
                assert_eq!(reader.$method().unwrap(), $value);
            }
        };
    }

    generate_read_num_le_test! {read_num_u8_val_le, u8, 11u8, read_u8}
    generate_read_num_le_test! {read_num_u8_max_le, u8, u8::MAX, read_u8}
    generate_read_num_le_test! {read_num_u8_min_le, u8, u8::MIN, read_u8}
    generate_read_num_le_test! {read_num_u16_val_le, u16, 11u16, read_u16}
    generate_read_num_le_test! {read_num_u16_max_le, u16, u16::MAX, read_u16}
    generate_read_num_le_test! {read_num_u16_min_le, u16, u16::MIN, read_u16}
    generate_read_num_le_test! {read_num_u32_val_le, u32, 11u32, read_u32}
    generate_read_num_le_test! {read_num_u32_max_le, u32, u32::MAX, read_u32}
    generate_read_num_le_test! {read_num_u32_min_le, u32, u32::MIN, read_u32}
    generate_read_num_le_test! {read_num_u64_val_le, u64, 11u64, read_u64}
    generate_read_num_le_test! {read_num_u64_max_le, u64, u64::MAX, read_u64}
    generate_read_num_le_test! {read_num_u64_min_le, u64, u64::MIN, read_u64}
    generate_read_num_le_test! {read_num_u128_val_le, u128, 11u128, read_u128}
    generate_read_num_le_test! {read_num_u128_max_le, u128, u128::MAX, read_u128}
    generate_read_num_le_test! {read_num_u128_min_le, u128, u128::MIN, read_u128}
    generate_read_num_le_test! {read_num_i8_val_le, i8, 11i8, read_i8}
    generate_read_num_le_test! {read_num_i8_max_le, i8, i8::MAX, read_i8}
    generate_read_num_le_test! {read_num_i8_min_le, i8, i8::MIN, read_i8}
    generate_read_num_le_test! {read_num_i16_val_le, i16, 11i16, read_i16}
    generate_read_num_le_test! {read_num_i16_max_le, i16, i16::MAX, read_i16}
    generate_read_num_le_test! {read_num_i16_min_le, i16, i16::MIN, read_i16}
    generate_read_num_le_test! {read_num_i32_val_le, i32, 11i32, read_i32}
    generate_read_num_le_test! {read_num_i32_max_le, i32, i32::MAX, read_i32}
    generate_read_num_le_test! {read_num_i32_min_le, i32, i32::MIN, read_i32}
    generate_read_num_le_test! {read_num_i64_val_le, i64, 11i64, read_i64}
    generate_read_num_le_test! {read_num_i64_max_le, i64, i64::MAX, read_i64}
    generate_read_num_le_test! {read_num_i64_min_le, i64, i64::MIN, read_i64}
    generate_read_num_le_test! {read_num_i128_val_le, i128, 11i128, read_i128}
    generate_read_num_le_test! {read_num_i128_max_le, i128, i128::MAX, read_i128}
    generate_read_num_le_test! {read_num_i128_min_le, i128, i128::MIN, read_i128}
    generate_read_num_le_test! {read_num_usize_val_le, usize, 11usize, read_usize}
    generate_read_num_le_test! {read_num_usize_max_le, usize, usize::MAX, read_usize}
    generate_read_num_le_test! {read_num_usize_min_le, usize, usize::MIN, read_usize}
    generate_read_num_le_test! {read_num_isize_val_le, isize, 11isize, read_isize}
    generate_read_num_le_test! {read_num_isize_max_le, isize, isize::MAX, read_isize}
    generate_read_num_le_test! {read_num_isize_min_le, isize, isize::MIN, read_isize}
    generate_read_num_le_test! {read_num_f32_val_le, f32, 11f32, read_f32}
    generate_read_num_le_test! {read_num_f32_max_le, f32, f32::MAX, read_f32}
    generate_read_num_le_test! {read_num_f32_min_le, f32, f32::MIN, read_f32}
    generate_read_num_le_test! {read_num_f64_val_le, f64, 11f64, read_f64}
    generate_read_num_le_test! {read_num_f64_max_le, f64, f64::MAX, read_f64}
    generate_read_num_le_test! {read_num_f64_min_le, f64, f64::MIN, read_f64}

    struct NEReader<T> {
        inner: T,
    }

    impl<T> Read for NEReader<T>
    where
        T: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.inner.read(buf)
        }
    }

    impl<T> ReadNum for NEReader<T>
    where
        T: Read,
    {
        type Reader = NativeEndianReader;
    }

    macro_rules! generate_read_num_ne_test {
        ($test_name: ident, $type: ty, $value: expr, $method: ident) => {
            #[test]
            fn $test_name() {
                let data = $value.to_ne_bytes();
                let mut reader = NEReader {
                    inner: Cursor::new(data),
                };
                assert_eq!(reader.$method().unwrap(), $value);
            }
        };
    }

    generate_read_num_ne_test! {read_num_u8_val_ne, u8, 11u8, read_u8}
    generate_read_num_ne_test! {read_num_u8_max_ne, u8, u8::MAX, read_u8}
    generate_read_num_ne_test! {read_num_u8_min_ne, u8, u8::MIN, read_u8}
    generate_read_num_ne_test! {read_num_u16_val_ne, u16, 11u16, read_u16}
    generate_read_num_ne_test! {read_num_u16_max_ne, u16, u16::MAX, read_u16}
    generate_read_num_ne_test! {read_num_u16_min_ne, u16, u16::MIN, read_u16}
    generate_read_num_ne_test! {read_num_u32_val_ne, u32, 11u32, read_u32}
    generate_read_num_ne_test! {read_num_u32_max_ne, u32, u32::MAX, read_u32}
    generate_read_num_ne_test! {read_num_u32_min_ne, u32, u32::MIN, read_u32}
    generate_read_num_ne_test! {read_num_u64_val_ne, u64, 11u64, read_u64}
    generate_read_num_ne_test! {read_num_u64_max_ne, u64, u64::MAX, read_u64}
    generate_read_num_ne_test! {read_num_u64_min_ne, u64, u64::MIN, read_u64}
    generate_read_num_ne_test! {read_num_u128_val_ne, u128, 11u128, read_u128}
    generate_read_num_ne_test! {read_num_u128_max_ne, u128, u128::MAX, read_u128}
    generate_read_num_ne_test! {read_num_u128_min_ne, u128, u128::MIN, read_u128}
    generate_read_num_ne_test! {read_num_i8_val_ne, i8, 11i8, read_i8}
    generate_read_num_ne_test! {read_num_i8_max_ne, i8, i8::MAX, read_i8}
    generate_read_num_ne_test! {read_num_i8_min_ne, i8, i8::MIN, read_i8}
    generate_read_num_ne_test! {read_num_i16_val_ne, i16, 11i16, read_i16}
    generate_read_num_ne_test! {read_num_i16_max_ne, i16, i16::MAX, read_i16}
    generate_read_num_ne_test! {read_num_i16_min_ne, i16, i16::MIN, read_i16}
    generate_read_num_ne_test! {read_num_i32_val_ne, i32, 11i32, read_i32}
    generate_read_num_ne_test! {read_num_i32_max_ne, i32, i32::MAX, read_i32}
    generate_read_num_ne_test! {read_num_i32_min_ne, i32, i32::MIN, read_i32}
    generate_read_num_ne_test! {read_num_i64_val_ne, i64, 11i64, read_i64}
    generate_read_num_ne_test! {read_num_i64_max_ne, i64, i64::MAX, read_i64}
    generate_read_num_ne_test! {read_num_i64_min_ne, i64, i64::MIN, read_i64}
    generate_read_num_ne_test! {read_num_i128_val_ne, i128, 11i128, read_i128}
    generate_read_num_ne_test! {read_num_i128_max_ne, i128, i128::MAX, read_i128}
    generate_read_num_ne_test! {read_num_i128_min_ne, i128, i128::MIN, read_i128}
    generate_read_num_ne_test! {read_num_usize_val_ne, usize, 11usize, read_usize}
    generate_read_num_ne_test! {read_num_usize_max_ne, usize, usize::MAX, read_usize}
    generate_read_num_ne_test! {read_num_usize_min_ne, usize, usize::MIN, read_usize}
    generate_read_num_ne_test! {read_num_isize_val_ne, isize, 11isize, read_isize}
    generate_read_num_ne_test! {read_num_isize_max_ne, isize, isize::MAX, read_isize}
    generate_read_num_ne_test! {read_num_isize_min_ne, isize, isize::MIN, read_isize}
    generate_read_num_ne_test! {read_num_f32_val_ne, f32, 11f32, read_f32}
    generate_read_num_ne_test! {read_num_f32_max_ne, f32, f32::MAX, read_f32}
    generate_read_num_ne_test! {read_num_f32_min_ne, f32, f32::MIN, read_f32}
    generate_read_num_ne_test! {read_num_f64_val_ne, f64, 11f64, read_f64}
    generate_read_num_ne_test! {read_num_f64_max_ne, f64, f64::MAX, read_f64}
    generate_read_num_ne_test! {read_num_f64_min_ne, f64, f64::MIN, read_f64}
}
