use napi::Result;
use napi_derive::napi;

use crate::endianness::Endianness;
use crate::stream::BinaryStream;

#[napi]
pub struct Float64();

#[napi]
impl Float64 {
  /**
   * Read a 64-bit floating point number (f64) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<f64> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Read 8 bytes from the stream
    let bytes = match stream.read(8) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err),
    };

    // Convert the bytes to f64 based on endianness
    match endian {
      Endianness::Big => Ok(f64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
      ])),
      Endianness::Little => Ok(f64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
      ])),
    }
  }

  /**
   * Write a 64-bit floating point number (f64) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: f64, endian: Option<Endianness>) -> Result<()> {
    // Provide a default endianness if not specified
    let endian = endian.unwrap_or(Endianness::Big);

    // Convert the f64 value to bytes based on endianness
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
