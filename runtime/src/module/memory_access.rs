use std::cell::Cell;
use it_traits::{SequentialReader, SequentialWriter};

#[macro_export]
macro_rules! value_der {
    ($self:expr, $offset:expr, @seq_start $($ids:tt),* @seq_end) => {
        [$($self.memory[$offset + $ids].get()),+]
    };

    ($self:expr, $offset:expr, 1) => {
        crate::value_der!($self, $offset, @seq_start 0 @seq_end);
    };

    ($self:expr, $offset:expr, 2) => {
        crate::value_der!($self, $offset, @seq_start 0, 1 @seq_end);
    };

    ($self:expr, $offset:expr, 4) => {
        crate::value_der!($self, $offset, @seq_start 0, 1, 2, 3 @seq_end);
    };

    ($self:expr, $offset:expr, 8) => {
        crate::value_der!($self, $offset, @seq_start 0, 1, 2, 3, 4, 5, 6, 7 @seq_end);
    };

    ($self:expr, $offset:expr, 16) => {
        crate::value_der!($self, $offset, @seq_start 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 @seq_end);
    };
}

#[macro_export]
macro_rules! read_ty {
    ($func_name:ident, $ty:ty, 1) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 1));

            self.offset.set(offset + 1);
            result
        }
    };

    ($func_name:ident, $ty:ty, 2) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 2));

            self.offset.set(offset + 2);
            result
        }
    };

    ($func_name:ident, $ty:ty, 4) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 4));

            self.offset.set(offset + 4);
            result
        }
    };

    ($func_name:ident, $ty:ty, 8) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 8));

            self.offset.set(offset + 8);
            result
        }
    };

    ($func_name:ident, $ty:ty, 16) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 16));

            self.offset.set(offset + 16);
            result
        }
    };
}

pub(crate) struct WasmerSequentialReader<'s> {
    pub memory: &'s [Cell<u8>],
    pub offset: Cell<usize>,
}

pub(crate) struct WasmerSequentialWriter<'s> {
    pub offset: usize,
    //pub view: MemoryView<'s, u8>,
    pub slice: &'s [Cell<u8>],
    pub current_offset: Cell<usize>,
}

impl SequentialReader for WasmerSequentialReader<'_> {
    fn read_bool(&self) -> bool {
        let value = self.read_u8();
        value != 0
    }

    read_ty!(read_u8, u8, 1);
    read_ty!(read_i8, i8, 1);
    read_ty!(read_u16, u16, 2);
    read_ty!(read_i16, i16, 2);
    read_ty!(read_u32, u32, 4);
    read_ty!(read_i32, i32, 4);
    read_ty!(read_f32, f32, 4);
    read_ty!(read_u64, u64, 8);
    read_ty!(read_i64, i64, 8);
    read_ty!(read_f64, f64, 8);
}

impl SequentialWriter for WasmerSequentialWriter<'_> {
    fn start_offset(&self) -> usize {
        self.offset
    }

    fn write_u8(&self, value: u8) {
        let offset = self.current_offset.get();
        self.slice[offset].set(value);
        self.current_offset.set(offset + 1);
    }

    fn write_u32(&self, value: u32) {
        self.write_bytes(&value.to_le_bytes());
    }

    fn write_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte)
        }
    }
}
