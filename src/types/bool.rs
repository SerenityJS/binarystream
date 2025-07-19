use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;

#[napi]
pub struct Bool();

#[napi]
impl Bool {
  /**
   * Read a boolean value (u8) from the BinaryStream.
  */
  #[napi]
  pub fn read(stream: &mut BinaryStream) -> Result<bool> {
    // Read a single byte from the stream
    let bytes = match stream.read(1) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Return true if the byte is non-zero, false otherwise
    Ok(bytes[0] != 0)
  }

  /**
   * Write a boolean value (u8) to the BinaryStream.
  */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: bool) -> Result<()> {
    // Convert the boolean value to a byte (1 for true, 0 for false)
    let byte = if value { 1 } else { 0 };

    // Write a single byte to the stream
    match stream.write(&[byte]) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}