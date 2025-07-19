use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;
use crate::types::unsigned::varint::VarInt;

#[napi]
pub struct VarString();

#[napi]
impl VarString {
  /**
   * Read a 32-bit variable length string from the BinaryStream.
  */
  #[napi]
  pub fn read(stream: &mut BinaryStream) -> Result<String> {
    // Read the length of the string (4 bytes)
    let length = match VarInt::read(stream) {
      Ok(len) => len,
      Err(err) => return Err(err),
    };

    // Read the string bytes from the stream
    let bytes = match stream.read(length as usize) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Convert the bytes to a UTF-8 string
    let utf8 = String::from_utf8_lossy(bytes);

    // Return the string
    Ok(utf8.into_owned())
  }

  /**
   * Write a 32-bit variable length string to the BinaryStream.
  */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: String) -> Result<()> {
    // Convert the string to bytes
    let bytes = value.into_bytes();

    // Write the length of the string (2 bytes)
    match VarInt::write(stream, bytes.len() as u32) {
      Ok(_) => {},
      Err(err) => return Err(err),
    };

    // Write the string bytes to the stream
    match stream.write(&bytes) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}