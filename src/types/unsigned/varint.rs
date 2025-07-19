use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;
use crate::types::unsigned::uint8::Uint8;

#[napi]
pub struct VarInt();

#[napi]
impl VarInt {
  /**
   * Read a unsigned 32-bit variable length integer (u32) from the BinaryStream.
  */
  #[napi]
  pub fn read(stream: &mut BinaryStream) -> Result<u32> {
    // Prepare the value and size variables
    let mut value = 0;
    let mut size = 0;

    // Read bytes until the continuation bit is not set
    loop {
      // Read a single byte from the stream
      let byte = match Uint8::read(stream) {
        Ok(byte) => byte,
        Err(err) => return Err(err)
      };
      
      // Update the value and size
      value |= (byte as u32 & 0x7F) << (size * 7);
      size += 1;

      // If the size exceeds 5, return an error
      if size > 5 {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "VarInt is too big"
        ))
      }

      // If the continuation bit is not set, break the loop
      if (byte & 0x80) != 0x80 {
        break;
      }
    }

    // Return the value as u32
    Ok(value)
  }

  /**
   * Write a unsigned 32-bit variable length integer (u32) to the BinaryStream.
  */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: u32) -> Result<()> {
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
        Ok(_) => {},
        Err(err) => return Err(err),
      };

      // If the value is zero, break the loop
      if value == 0 {
        break;
      }
    }

    Ok(())
  }
}