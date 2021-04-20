/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::cell::Cell;

pub(crate) struct MemoryReader<'m> {
    pub(self) memory: &'m [Cell<u8>],
}

pub(crate) struct SequentialReader<'r, 'm> {
    reader: &'r MemoryReader<'m>,
    offset: Cell<usize>,
}

macro_rules! value_der {
    ($self:expr, $offset:expr, @seq_start $($ids:tt),* @seq_end) => {
        [$(std::cell::Cell::get($self.reader.memory[$offset + $ids])),+]
    };

    ($self:expr, $offset:expr, 1) => {
        value_der!($self, $offset, @seq_start 0 @seq_end);
    };

    ($self:expr, $offset:expr, 2) => {
        value_der!($self, $offset, @seq_start 0, 1 @seq_end);
    };

    ($self:expr, $offset:expr, 4) => {
        value_der!($self, $offset, @seq_start 0, 1, 2, 3 @seq_end);
    };

    ($self:expr, $offset:expr, 8) => {
        value_der!($self, $offset, @seq_start 0, 1, 2, 3, 4, 5, 6, 7 @seq_end);
    };
}

macro_rules! read_ty {
    ($func_name:ident, $ty:ty, 1) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 1));

            self.offset.set(offset + 1);
            result
        }
    };

    ($func_name:ident, $ty:ty, 2) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 2));

            self.offset.set(offset + 2);
            result
        }
    };

    ($func_name:ident, $ty:ty, 4) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 4));

            self.offset.set(offset + 4);
            result
        }
    };

    ($func_name:ident, $ty:ty, 8) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 8));

            self.offset.set(offset + 8);
            result
        }
    };
}

impl<'m> MemoryReader<'m> {
    pub(crate) fn new(memory: &'m [Cell<u8>]) -> Self {
        Self { memory }
    }

    pub(crate) fn sequential_reader(&self, offset: usize) -> SequentialReader<'_, '_> {
        SequentialReader::new(&self, offset)
    }

    pub(self) fn memory(&self) -> &[Cell<u8>] {
        self.memory
    }
}

impl<'r, 'm> SequentialReader<'r, 'm> {
    pub(crate) fn new(reader: &'r MemoryReader<'m>, offset: usize) -> Self {
        let offset = Cell::new(offset);
        Self { reader, offset }
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
