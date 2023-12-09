use napi_derive::napi;
use napi::bindgen_prelude::*;

#[napi]
pub struct BinaryStream {
  /**
   * **binary**
   * 
   * The binary data of the stream.
  */
  pub binary: Vec<u8>,
  /**
   * **offset**
   * 
   * The current offset of the stream.
   */
  pub offset: u32,
}

#[napi]
impl BinaryStream {
  /**
   * **BinaryStream**
   * 
   * Creates a new BinaryStream with an optional JavaScript Buffer.
  */
  #[napi(constructor)]
  pub fn new(buffer: Option<Buffer>, offset: Option<u32>) -> Self {
    let bin = buffer.unwrap_or(Buffer::from(vec![]));
    let offset = offset.unwrap_or(0);

    return BinaryStream {
      binary: bin.as_ref().to_vec(),
      offset,
    }
  }

  /**
   * **from**
   * 
   * Creates a new BinaryStream from a binary vector.
  */
  #[napi(factory)]
  pub fn from(binary: Vec<u8>, offset: Option<u32>) -> Self {
    let offset = offset.unwrap_or(0);

    return BinaryStream {
      binary,
      offset,
    }
  }

  /**
   * **fromBuffer**
   * 
   * Creates a new BinaryStream from a JavaScript Buffer.
  */
  #[napi(factory)]
  pub fn from_buffer(buffer: Buffer, offset: Option<u32>) -> Self {
    let offset = offset.unwrap_or(0);

    return BinaryStream {
      binary: buffer.as_ref().to_vec(),
      offset,
    }
  }

  /**
   * **read**
   * 
   * Reads a number of bytes from the stream.
  */
  #[napi]
  pub fn read(&mut self, length: u32) -> Vec<u8> {
    let start = self.offset as usize;
    let end = (self.offset + length) as usize;
    self.offset += length;

    return self.binary[start..end].to_vec()
  }

  /**
   * **readBuffer**
   * 
   * Reads a number of bytes from the stream and returns a JavaScript Buffer.
  */
  #[napi]
  pub fn read_buffer(&mut self, length: u32) -> Buffer {
    let bytes = self.read(length);

    return Buffer::from(bytes)
  }

  /**
   * **write**
   * 
   * Writes a number of bytes to the stream.
  */
  #[napi]
  pub fn write(&mut self, data: Vec<u8>) {
    self.binary.extend(data);
  }

  /**
   * ***writeBuffer*
   * 
   * Writes a JavaScript Buffer to the stream.
  */
  #[napi]
  pub fn write_buffer(&mut self, data: Buffer) {
    data.as_ref().to_vec();
  }

  /**
   * **readRemaining**
   * 
   * Reads the remaining bytes from the stream.
  */
  #[napi]
  pub fn read_remaining(&mut self) -> Vec<u8> {
    let start = self.offset as usize;
    let end = self.binary.len();
    self.offset = end as u32;

    return self.binary[start..end].to_vec()
  }

  /**
   * **readRemainingBuffer**
   * 
   * Reads the remaining bytes from the stream and returns a JavaScript Buffer.
  */
  #[napi]
  pub fn read_remaining_buffer(&mut self) -> Buffer {
    let bytes = self.read_remaining();

    return Buffer::from(bytes)
  }

  /**
   * **skip**
   * 
   * Skips a number of bytes from the stream.
  */
  #[napi]
  pub fn skip(&mut self, length: u32) -> u32 {
    self.offset += length;

    return self.offset
  }

  /**
   * **cursorAtEnd**
   * 
   * Checks if the cursor is at the end of the stream.
  */
  #[napi]
  pub fn cursor_at_end(&self) -> bool {
    return self.offset == self.binary.len() as u32
  }

  /**
   * **cursorAtStart**
   * 
   * Checks if the cursor is at the start of the stream.
  */
  #[napi]
  pub fn cursor_at_start(&self) -> bool {
    return self.offset == 0
  }

  /**
   * **getBuffer**
   * 
   * Gets the binary as a JavaScript Buffer.
  */
  #[napi]
  pub fn get_buffer(&self) -> Buffer {
    return Buffer::from(self.binary.clone())
  }
}