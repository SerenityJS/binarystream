use napi::bindgen_prelude::BigInt;
use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;
use crate::types::unsigned::uint8::Uint8;

#[napi]
pub struct VarLong();

impl VarLong {
  pub fn read(stream: &mut BinaryStream) -> Result<u64> {
    // Prepare the value and size variables
    let mut value = 0u64;
    let mut size = 0;

    // Read bytes until the continuation bit is not set
    loop {
      // Read a single byte from the stream
      let byte = match Uint8::read(stream) {
        Ok(byte) => byte,
        Err(err) => return Err(err),
      };

      // Update the value and size
      value |= (byte as u64 & 0x7F) << (7 * size);
      size += 1;

      // If the size exceeds 10, return an error
      if size > 10 {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "VarLong is too big",
        ));
      }

      // If the continuation bit is not set, break the loop
      if (byte & 0x80) != 0x80 {
        break;
      }
    }

    Ok(value)
  }

  pub fn write(stream: &mut BinaryStream, value: u64) -> Result<()> {
    // Prepare the value to write
    let mut value = value;

    // Write bytes until the value is zero
    loop {
      // Get the byte to write
      let mut byte = (value & 0x7F) as u8;
      value >>= 7;

      // If there are more bytes to write, set the continuation bit
      if value != 0 {
        byte |= 0x80;
      }

      // Write the byte to the stream
      match Uint8::write(stream, byte) {
        Ok(_) => {}
        Err(err) => return Err(err),
      }

      // If the value is zero, break the loop
      if value == 0 {
        break;
      }
    }

    Ok(())
  }
}

#[napi]
impl VarLong {
  /**
   * Read a unsigned 64-bit variable length integer (u64) from the BinaryStream.
   */
  #[napi(js_name = "read")]
  pub fn read_napi(stream: &mut BinaryStream) -> Result<BigInt> {
    // Read a variable length integer from the stream
    let value = match VarLong::read(stream) {
      Ok(value) => value,
      Err(err) => return Err(err),
    };

    // Return the value as BigInt
    Ok(BigInt::from(value))
  }

  /**
   * Write a unsigned 64-bit variable integer (u64) to the BinaryStream.
   */
  #[napi(js_name = "write")]
  pub fn write_napi(stream: &mut BinaryStream, value: BigInt) -> Result<()> {
    // Convert BigInt to u64
    let value = value.get_u64().1;

    // Write the value to the stream
    match VarLong::write(stream, value) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}
