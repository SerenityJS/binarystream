use napi::Result;
use napi_derive::napi;

use crate::endianness::Endianness;
use crate::stream::BinaryStream;

#[napi]
pub struct Uint16();

#[napi]
impl Uint16 {
  /**
   * Read a unsigned 16-bit integer (u16) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<u16> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Read 2 bytes from the stream
    let bytes = match stream.read(2) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Convert the bytes to u16 based on endianness
    match endian {
      Endianness::Big => Ok(u16::from_be_bytes([bytes[0], bytes[1]])),
      Endianness::Little => Ok(u16::from_le_bytes([bytes[0], bytes[1]])),
    }
  }

  /**
   * Write a unsigned 16-bit integer (u16) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: u16, endian: Option<Endianness>) -> Result<()> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Convert the u16 value to bytes based on endianness
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
