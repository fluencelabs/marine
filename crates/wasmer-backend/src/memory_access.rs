use std::cell::Cell;
//use wasmer_it::interpreter::wasm::structures::{SequentialReader, SequentialWriter};
use it_memory_traits::{MemoryReadable, MemoryWritable};

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

pub struct WasmerSequentialReader<'s> {
    pub memory: &'s [Cell<u8>],
    pub offset: Cell<usize>,
}

pub struct WasmerSequentialWriter<'s> {
    pub offset: usize,
    pub slice: &'s [Cell<u8>],
    pub current_offset: Cell<usize>,
}

