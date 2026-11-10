use crate::FromConfig;

use crate::utils::Sizeof;
use crate::registers::IntoIntegerRegister;

use crate::Vec;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryCopyError {
    #[error("Value larger than available memory")]
    InvalidValueSize,
    #[error("Copying value to memory offset would overflow end of memory")]
    InvalidOffset,
    #[error("Memory in invalid state")]
    InvalidMemoryState
}

pub struct Memory {
    pub base_ptr: *mut u8,
    pub len: usize,
    pub backing_buf: Vec<u8>
}

impl FromConfig for Memory {
    fn from_config(config: &crate::config::Config) -> Self {
        let mut backing_buf = Vec::with_capacity(config.memory_len as usize);
        let base_ptr = backing_buf.as_mut_ptr();
        let len = backing_buf.len();

        Memory { base_ptr, len, backing_buf }
    }
}

impl core::ops::Index<usize> for Memory {
    type Output = u8;

    fn index(&self, mut index: usize) -> &Self::Output {
        while index > self.len {
            index -= self.len; // Simulate memory wrapping
        }

        unsafe { (self.base_ptr.add(index)).as_ref_unchecked() }
    }
}

impl core::ops::Index<crate::registers::IntegerRegister> for Memory {
    type Output = u8;

    fn index(&self, mut index: crate::registers::IntegerRegister) -> &Self::Output {
        if index.larger_than_usize() {
            while index > usize::MAX.into_integer_register() {
                index -= usize::MAX.into_integer_register()
            }
        }

        unsafe { (self.base_ptr.add(index.to_usize())).as_ref_unchecked() }
    }
}

impl Memory {
    fn copy_bytes_at_offset(&mut self, bytes: &[u8], offset: usize) -> Result<(), MemoryCopyError> {
        if self.backing_buf.is_empty() {
            return Err(MemoryCopyError::InvalidMemoryState);
        }

        if self.base_ptr.is_null() {
            // Try to recover
            self.base_ptr = self.backing_buf.as_mut_ptr();
        }

        if self.len != self.backing_buf.len() {
            self.len = self.backing_buf.len();
        }

        if bytes.len() > self.len {
            return Err(MemoryCopyError::InvalidValueSize);
        }

        if offset + bytes.len() > self.len {
            return Err(MemoryCopyError::InvalidOffset);
        }

        &mut self.backing_buf[offset..(offset + bytes.len())].copy_from_slice(bytes);

        Ok(())
    }

    fn copy_sized_value_at_offset<T: Sized>(&mut self, val: T, offset: usize) -> Result<(), MemoryCopyError> {
        if self.backing_buf.is_empty() {
            return Err(MemoryCopyError::InvalidMemoryState);
        }

        if self.base_ptr.is_null() {
            // Try to recover
            self.base_ptr = self.backing_buf.as_mut_ptr();
        }

        if self.len != self.backing_buf.len() {
            self.len = self.backing_buf.len();
        }

        let val_size: usize = core::mem::size_of::<T>();

        if val_size > self.len {
            return Err(MemoryCopyError::InvalidValueSize);
        }

        if offset + val_size > self.len {
            return Err(MemoryCopyError::InvalidOffset);
        }

        let mut bytes = &val as *const T as *const u8;
        let byte_slice = unsafe { core::slice::from_raw_parts(bytes, val_size) };

        &mut self.backing_buf[offset..(offset + val_size)].copy_from_slice(byte_slice);

        unsafe {
            let mut bytes = &val as *const T as *const u8;
            let mut ptr = self.base_ptr.add(offset);

            for _ in 0..val_size {
                ptr.write_unaligned(*bytes);
                bytes = bytes.add(1);
                ptr = ptr.add(1);
            }
        }

        Ok(())
    }
}