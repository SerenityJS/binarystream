use napi::bindgen_prelude::{BigInt, FromNapiValue};
use napi_derive::napi;
use napi::Result;
use crate::binary::BinaryStream;
use crate::types::VarLong;

#[napi]
#[derive(Clone)]
/**
 * **ZigZong**
 * 
 * Represents a 64 bit ( 8 bytes ) zigzag encoded signed variable length integer. ( -9223372036854775808 to 9223372036854775807 )
*/
pub struct ZigZong {}

#[napi]
impl ZigZong {
  #[napi]
  /**
   * **read**
   * 
   * Reads a 64 bit ( 8 bytes ) zigzag encoded signed variable length integer from the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  pub fn read(stream: &mut BinaryStream) -> Result<BigInt> {
    let value = match VarLong::read(stream) {
      Ok(value) => value,
      Err(err) => return Err(err)
    };

    let value = value.get_u64().1;

    let value = ((value >> 1) as i64) ^ (-((value & 1) as i64));
    let signed = value < 0;

    let value = match signed {
      true => (-value) as u64,
      false => value as u64,
    };
    
    let value = napi::bindgen_prelude::BigInt {
      sign_bit: signed,
      words: vec![value],
    };

    Ok(value)
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a 64 bit ( 8 bytes ) zigzag encoded signed variable length integer to the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  pub fn write(stream: &mut BinaryStream, value: BigInt) {
    let value_ = value.get_i64().0;
    let signed = value.sign_bit;

    let value = match signed {
      true => -value_,
      false => value_
    };

    let value = (value << 1) ^ (value >> 63);

    VarLong::write(stream, BigInt::from(value));
  }
}

impl FromNapiValue for ZigZong {
  unsafe fn from_napi_value(_: napi::sys::napi_env, _: napi::sys::napi_value) -> Result<Self> {
    Ok(ZigZong {})
  }
}