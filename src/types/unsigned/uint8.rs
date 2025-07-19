use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;

#[napi]
pub struct Uint8();

#[napi]
impl Uint8 {
  /**
   * Read a unsigned 8-bit integer (u8) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream) -> Result<u8> {
    // Read a single byte from the stream
    let bytes = match stream.read(1) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Return the first byte as u8
    Ok(u8::from_be_bytes([bytes[0]]))
  }

  /**
   * Write a unsigned 8-bit integer (u8) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: u8) -> Result<()> {
    // Convert the u8 value to bytes
    let bytes = value.to_be_bytes();

    // Write a single byte to the stream
    match stream.write(&bytes) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}
