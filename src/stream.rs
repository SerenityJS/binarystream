use napi_derive::napi;

use napi::{bindgen_prelude::{BufferSlice, JsValuesTuple}, Env, Result};

#[napi]
pub struct BinaryStream<'env> {
  /**
   * The buffer reference that this stream will read/write to.
  */
  buffer: BufferSlice<'env>,

  /**
   * The current offset in the buffer where the next read/write will occur.
  */
  offset: usize,
}

#[napi]
impl<'env> BinaryStream<'env> {
  pub fn write(&mut self, data: &[u8]) -> Result<()> {
    // Check if the write will exceed the buffer length
    if self.offset + data.len() > self.buffer.len() {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Write exceeds buffer length",
      ));
    }

    // Write the slice to the buffer
    for (i, &byte) in data.iter().enumerate() {
      self.buffer[self.offset + i] = byte;
    }

    // Update the offset
    self.offset += data.len();
    Ok(())
  }

  pub fn read(&mut self, length: usize) -> Result<&[u8]> {
    // Check if the read will exceed the buffer length
    if self.offset + length > self.buffer.len() {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Read exceeds buffer length",
      ));
    }

    // Create a slice of the buffer to return
    let slice = &self.buffer[self.offset..self.offset + length];

    // Update the offset
    self.offset += length;

    // Return the slice
    Ok(slice)
  }
}

#[napi]
impl<'env> BinaryStream<'env> {
  #[napi(constructor)]
  pub fn new(buffer: BufferSlice<'env>, offset: Option<u32>) -> Self {
    let offset = offset.unwrap_or(0) as usize;

    BinaryStream { buffer, offset }
  }

  /**
   * Returns the current offset in the stream.
  */
  #[napi(js_name = "getOffset")]
  pub fn get_offset_napi(&self) -> Result<u32> {
    // Cast the offset to u32 and return it
    Ok(self.offset as u32)
  }

  /**
   * Sets the current offset in the stream.
   * If the offset is out of bounds, it returns an error.
  */
  #[napi(js_name = "setOffset")]
  pub fn set_offset_napi(&mut self, offset: u32) -> Result<()> {
    // Check if the offset is within the bounds of the buffer
    if offset as usize > self.buffer.len() {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Offset is out of bounds",
      ));
    }

    // Update the offset
    self.offset = offset as usize;
    Ok(())
  }

  #[napi(js_name = "write")]
  pub fn write_napi(&mut self, data: BufferSlice<'env>) -> Result<()> {
    // Check if the write will exceed the buffer length
    if self.offset + data.len() > self.buffer.len() {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Write exceeds buffer length",
      ));
    }

    // Iterate over the data and write it to the buffer
    for (i, &byte) in data.iter().enumerate() {
      self.buffer[self.offset + i] = byte;
    }

    // Update the offset
    self.offset += data.len();
    Ok(())
  }

    #[napi(js_name = "read")]
  pub fn read_napi(&mut self, length: u32) -> Result<BufferSlice<'env>> {
    // Check if the read will exceed the buffer length
    if self.offset + length as usize > self.buffer.len() {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Read exceeds buffer length",
      ));
    }

    // Create a slice of the buffer to return
    let slice = &self.buffer[self.offset..self.offset + length as usize];

    // Update the offset
    self.offset += length as usize;

    // Get the environment from the buffer
    let env = &Env::from_raw(self.buffer.env());

    // Create a BufferSlice from the slice
    let buffer = BufferSlice::from_data(env, slice)
      .expect("Failed to create BufferSlice from data");

    // Return the slice as a BufferSlice
    Ok(buffer)
  }

  /**
   * Checks if the stream has reached the end of the buffer.
  */
  #[napi]
  pub fn feof(&self) -> bool {
    // Check if the current offset is at the end of the buffer
    self.offset >= self.buffer.len()
  }

  /**
   * Resets the stream's offset to the beginning of the buffer.
  */
  #[napi]
  pub fn reset(&mut self) {
    // Reset the offset to 0
    self.offset = 0;
  }
}