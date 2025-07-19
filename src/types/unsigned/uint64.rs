use napi::bindgen_prelude::BigInt;
use napi::Result;
use napi_derive::napi;

use crate::endianness::Endianness;
use crate::stream::BinaryStream;

#[napi]
pub struct Uint64();

#[napi]
impl Uint64 {
  /**
   * Read a unsigned 64-bit integer (u64) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<BigInt> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Read 8 bytes from the stream
    let bytes = match stream.read(8) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Convert the bytes to u64 based on endianness
    let value = match endian {
      Endianness::Big => u64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
      ]),
      Endianness::Little => u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
      ]),
    };

    // Return the value as BigInt
    Ok(BigInt::from(value))
  }

  /**
   * Write a unsigned 64-bit integer (u64) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: BigInt, endian: Option<Endianness>) -> Result<()> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Convert the u64 value to bytes based on endianness
    let bytes = match endian {
      Endianness::Big => value.get_u64().1.to_be_bytes(),
      Endianness::Little => value.get_u64().1.to_le_bytes(),
    };

    // Write the bytes to the stream
    match stream.write(&bytes) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}
