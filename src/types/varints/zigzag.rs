use napi::bindgen_prelude::FromNapiValue;
use napi_derive::napi;
use napi::Result;
use crate::binary::BinaryStream;
use crate::types::VarInt;

#[napi]
#[derive(Clone)]
/**
 * **ZigZag**
 * 
 * Represents a 32 bit ( 4 bytes ) zigzag encoded signed variable length integer. ( -2147483648 to 2147483647 )
*/
pub struct ZigZag {}

#[napi]
impl ZigZag {
  #[napi]
  /**
   * **read**
   * 
   * Reads a 32 bit ( 4 bytes ) zigzag encoded signed variable length integer from the stream. ( -2147483648 to 2147483647 )
  */
  pub fn read(stream: &mut BinaryStream) -> Result<i32> {
    let value = match VarInt::read(stream) {
      Ok(value) => value,
      Err(err) => return Err(err)
    };

    Ok(((value >> 1) as i32) ^ (-((value & 1) as i32)))
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a 32 bit ( 4 bytes ) zigzag encoded signed variable length integer to the stream. ( -2147483648 to 2147483647 )
  */
  pub fn write(stream: &mut BinaryStream, value: i32) {
    let value = ((value << 1) ^ (value >> 31)) as u32;
    VarInt::write(stream, value);
  }
}


impl FromNapiValue for ZigZag {
  unsafe fn from_napi_value(_: napi::sys::napi_env, _: napi::sys::napi_value) -> Result<Self> {
    Ok(ZigZag {})
  }
}