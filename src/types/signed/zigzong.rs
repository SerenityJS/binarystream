use napi::bindgen_prelude::BigInt;
use napi::Result;
use napi_derive::napi;

use crate::stream::BinaryStream;
use crate::types::unsigned::varlong::VarLong;

#[napi]
pub struct ZigZong();

impl ZigZong {
  pub fn read(stream: &mut BinaryStream) -> Result<i64> {
    // Read a variable length integer from the stream
    let mut value = match VarLong::read(stream) {
      Ok(value) => value as i64,
      Err(err) => return Err(err),
    };

    // Convert the value from ZigZag encoding
    value = ((value >> 1) as i64) ^ (-((value & 1) as i64));

    // Return the value as i64
    Ok(value)
  }

  pub fn write(stream: &mut BinaryStream, value: i64) -> Result<()> {
    // Convert the value to ZigZag encoding
    let value = ((value << 1) ^ (value >> 63)) as u64;

    // Write the variable length integer to the stream
    match VarLong::write(stream, value) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}

#[napi]
impl ZigZong {
  /**
   * Read a signed 64-bit variable length integer (i64) from the BinaryStream.
   */
  #[napi(js_name = "read")]
  pub fn read_napi(stream: &mut BinaryStream) -> Result<BigInt> {
    // Read a variable length integer from the stream
    let value = match ZigZong::read(stream) {
      Ok(value) => value,
      Err(err) => return Err(err),
    };

    // Return the value as BigInt
    Ok(BigInt::from(value))
  }

  /**
   * Write a signed 64-bit variable integer (i64) to the BinaryStream.
   */
  #[napi(js_name = "write")]
  pub fn write_napi(stream: &mut BinaryStream, value: BigInt) -> Result<()> {
    // Convert BigInt to i64
    let value = value.get_i64().0;

    // Write the value to the stream
    match ZigZong::write(stream, value) {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}
