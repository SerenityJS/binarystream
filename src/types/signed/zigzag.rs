use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;
use crate::types::unsigned::varint::VarInt;

#[napi]
pub struct ZigZag();

#[napi]
impl ZigZag {
  /**
   * Read a unsigned 32-bit variable length integer (i32) from the BinaryStream.
   */
  #[napi]
  pub fn read(stream: &mut BinaryStream) -> Result<i32> {
    // Read a variable length integer from the stream
    let mut value = match VarInt::read(stream) {
      Ok(value) => value as i32,
      Err(err) => return Err(err),
    };

    // Convert the value from ZigZag encoding
    value = ((value >> 1) as i32) ^ (-((value & 1) as i32));

    // Return modified value
    Ok(value)
  }

  /**
   * Write a unsigned 32-bit variable length integer (i32) to the BinaryStream.
   */
  #[napi]
  pub fn write(stream: &mut BinaryStream, value: i32) -> Result<()> {
    // Convert the value to ZigZag encoding
    let value = ((value << 1) ^ (value >> 31)) as u32;

    // Write the variable length integer to the stream
    match VarInt::write(stream, value) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}
