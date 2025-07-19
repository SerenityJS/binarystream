use napi::Result;
use napi_derive::napi;

use crate::endianness::Endianness;
use crate::stream::BinaryStream;

#[napi]
pub struct Int32();

#[napi]
impl Int32 {
  /**
   * Read a unsigned 32-bit integer (i32) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<i32> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Read 4 bytes from the stream
    let bytes = match stream.read(4) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Convert the bytes to i32 based on endianness
    match endian {
      Endianness::Big => Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
      Endianness::Little => Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
    }
  }

  /**
   * Write a unsigned 32-bit integer (i32) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: i32, endian: Option<Endianness>) -> Result<()> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Convert the i32 value to bytes based on endianness
    let bytes = match endian {
      Endianness::Big => value.to_be_bytes(),
      Endianness::Little => value.to_le_bytes(),
    };

    // Write the bytes to the stream
    match stream.write(&bytes) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}
