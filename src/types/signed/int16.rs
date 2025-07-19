use napi::Result;
use napi_derive::napi;

use crate::endianness::Endianness;
use crate::stream::BinaryStream;

#[napi]
pub struct Int16();

#[napi]
impl Int16 {
  /**
   * Read a unsigned 16-bit integer (i16) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<i16> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Read 2 bytes from the stream
    let bytes = match stream.read(2) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Convert the bytes to i16 based on endianness
    match endian {
      Endianness::Big => Ok(i16::from_be_bytes([bytes[0], bytes[1]])),
      Endianness::Little => Ok(i16::from_le_bytes([bytes[0], bytes[1]])),
    }
  }

  /**
   * Write a unsigned 16-bit integer (i16) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: i16, endian: Option<Endianness>) -> Result<()> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Convert the i16 value to bytes based on endianness
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
