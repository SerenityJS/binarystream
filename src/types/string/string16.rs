use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;
use crate::endianness::Endianness;
use crate::types::unsigned::uint16::Uint16;

#[napi]
pub struct String16();

#[napi]
impl String16 {
  /**
   * Read a 16-bit string from the BinaryStream.
  */
  #[napi]
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<String> {
    // Read the length of the string (2 bytes)
    let length = match Uint16::read(stream, endian) {
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
   * Write a 16-bit string to the BinaryStream.
  */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: String, endian: Option<Endianness>) -> Result<()> {
    // Convert the string to bytes
    let bytes = value.into_bytes();

    // Write the length of the string (2 bytes)
    match Uint16::write(stream, bytes.len() as u16, endian) {
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