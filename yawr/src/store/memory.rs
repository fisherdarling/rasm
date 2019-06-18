use std::fmt;
use std::io::Write;
// use byteorder::{WriteBytesExt, ByteOrder};

use crate::runtime::interpreter::Interpreter;
use crate::types::{
    index::{Align, Offset},
    Data, Value,
};

use crate::error::Error;

pub static PAGE_SIZE: usize = 1 << 16;
pub static MAX_SIZE: usize = (1 << 30) * 4; // 4 GiB
pub static MAX_PAGES: usize = MAX_SIZE / PAGE_SIZE;

#[derive(Clone, PartialEq)]
pub struct MemInst {
    data: Vec<u8>,
    min: u32,
    max: Option<u32>,
}

impl Default for MemInst {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            min: 1,
            max: Some(MAX_PAGES as u32),
        }
    }
}

impl fmt::Debug for MemInst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MemInst {{ data: {:?} Pages, min: {:?}, max: {:?} }}",
            self.data.len() / PAGE_SIZE,
            self.min,
            self.max
        )
    }
}

impl MemInst {
    pub fn new(min: u32, max: Option<u32>) -> Self {
        MemInst {
            data: Vec::new(),
            min,
            max,
        }
    }
    pub fn init(&mut self, data_init: Option<Vec<Data>>) -> Result<(), Error> {
        self.data.resize(self.min as usize * PAGE_SIZE, 0);

        if let Some(data_init) = data_init {
            for data in data_init {
                let idx = data.0;
                let offset = Interpreter::get_index(data.1).ok_or(Error::OffsetExpression)?;
                let bytes = data.2;

                let mut buffer: &mut [u8] = &mut self.data[offset..];
                // TODO: Convert IO Errors
                buffer.write_all(&bytes).unwrap();
                buffer.flush().unwrap();
            }
        }

        Ok(())
    }

    pub fn load_i32(&self, align: Align, offset: Offset) -> i32 {
        // Align is always 4? Ignore?
        let offset = offset.as_usize();

        let data = [
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ];

        let value = i32::from_le_bytes(data);

        // log::debug!("[MEMORY] LOAD I32: {}", value);

        value
    }

    pub fn load_i64(&self, align: Align, offset: Offset) -> i64 {
        // Align is always 4? Ignore?
        let offset = offset.as_usize();

        let data = [
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
            self.data[offset + 4],
            self.data[offset + 5],
            self.data[offset + 6],
            self.data[offset + 7],
        ];

        i64::from_le_bytes(data)
    }

    pub fn load_f32(&self, align: Align, offset: Offset) -> f32 {
        let bits = self.load_i32(align, offset);

        f32::from_bits(bits as u32)
    }

    pub fn load_f64(&self, align: Align, offset: Offset) -> f64 {
        let bits = self.load_i64(align, offset);

        f64::from_bits(bits as u64)
    }

    pub fn load_i32_8_s(&self, align: Align, offset: Offset) -> i32 {
        let offset = offset.as_usize();

        let data = self.data[offset] as i8;

        data as i32
    }

    pub fn load_i32_8_u(&self, align: Align, offset: Offset) -> i32 {
        let offset = offset.as_usize();

        let data = self.data[offset];

        data as i32
    }

    pub fn load_i32_16_s(&self, align: Align, offset: Offset) -> i32 {
        let offset = offset.as_usize();

        let data = [self.data[offset], self.data[offset + 1]];

        i16::from_le_bytes(data) as i32
    }

    pub fn load_i32_16_u(&self, align: Align, offset: Offset) -> i32 {
        let offset = offset.as_usize();

        let data = [self.data[offset], self.data[offset + 1]];

        u16::from_le_bytes(data) as i32
    }

    pub fn load_i64_8_s(&self, align: Align, offset: Offset) -> i64 {
        let data = self.load_i32_8_s(align, offset);

        data as i64
    }

    pub fn load_i64_8_u(&self, align: Align, offset: Offset) -> i64 {
        let data = self.load_i32_8_u(align, offset);

        data as i64
    }

    pub fn load_i64_16_s(&self, align: Align, offset: Offset) -> i64 {
        let data = self.load_i32_16_s(align, offset);

        data as i64
    }

    pub fn load_i64_16_u(&self, align: Align, offset: Offset) -> i64 {
        let data = self.load_i32_16_u(align, offset);

        data as i64
    }

    pub fn load_i64_32_s(&self, align: Align, offset: Offset) -> i64 {
        let data = self.load_i32(align, offset);

        data as i64
    }

    pub fn load_i64_32_u(&self, align: Align, offset: Offset) -> i64 {
        let data = self.load_i32(align, offset) as u32;

        data as i64
    }

    pub fn store_i32(&mut self, align: Align, offset: Offset, value: i32) {
        let offset = offset.as_usize();

        let mut buffer: &mut [u8] = &mut self.data[offset..offset + 4];

        buffer.write(&value.to_le_bytes()).unwrap();
        buffer.flush().unwrap();
    }

    pub fn store_i64(&mut self, align: Align, offset: Offset, value: i64) {
        let offset = offset.as_usize();

        let mut buffer: &mut [u8] = &mut self.data[offset..offset + 8];

        buffer.write(&value.to_le_bytes()).unwrap();
        buffer.flush().unwrap();
    }

    pub fn store_f32(&mut self, align: Align, offset: Offset, value: f32) {
        self.store_i32(align, offset, value.to_bits() as i32);
    }

    pub fn store_f64(&mut self, align: Align, offset: Offset, value: f64) {
        self.store_i64(align, offset, value.to_bits() as i64);
    }

    pub fn store_i32_8(&mut self, align: Align, offset: Offset, value: i32) {
        self.store_i32(align, offset, value & 0xFF)
    }

    pub fn store_i32_16(&mut self, align: Align, offset: Offset, value: i32) {
        self.store_i32(align, offset, value & 0xFF_FF)
    }

    pub fn store_i64_8(&mut self, align: Align, offset: Offset, value: i64) {
        self.store_i64(align, offset, value & 0xFF)
    }

    pub fn store_i64_16(&mut self, align: Align, offset: Offset, value: i64) {
        self.store_i64(align, offset, value & 0xFF_FF)
    }

    pub fn store_i64_32(&mut self, align: Align, offset: Offset, value: i64) {
        self.store_i64(align, offset, value & 0xFF_FF_FF_FF)
    }

    pub fn mem_size(&self) -> usize {
        let size = self.data.len() / PAGE_SIZE;

        size
    }

    pub fn mem_grow(&mut self, num_pages: i32) -> Result<i32, Error> {
        let prev_size = self.mem_size();
        let num_pages = num_pages as usize;

        if prev_size + num_pages >= MAX_PAGES {
            return Err(Error::MemoryExceeded);
        }

        self.data
            .try_reserve_exact(num_pages * PAGE_SIZE)
            .map_err(Error::MemoryGrow)?;

        Ok(prev_size as i32)
    }
}
