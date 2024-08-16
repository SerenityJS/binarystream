use napi_derive::napi;
use napi::Result;
use napi::bindgen_prelude::{BigInt, FromNapiValue};
use crate::binary::{ BinaryStream, Endianness };
use crate::types::Uint64;

#[napi]
#[derive(Clone)]
/**
 * **ULong**
 * 
 * Represents an unsigned 64-bit ( 8 bytes ) integer. ( 0 to 18446744073709551615 )
*/
pub struct ULong {}

#[napi]
impl ULong {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 64-bit ( 8 bytes ) integer from the stream. ( 0 to 18446744073709551615 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<BigInt> {
    Uint64::read(stream, endian)
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 64-bit ( 8 bytes ) integer to the stream. ( 0 to 18446744073709551615 )
  */
  pub fn write(stream: &mut BinaryStream, value: BigInt, endian: Option<Endianness>) {
    Uint64::write(stream, value, endian);
  }
}

impl FromNapiValue for ULong {
  unsafe fn from_napi_value(_: napi::sys::napi_env, _: napi::sys::napi_value) -> Result<Self> {
    Ok(ULong {})
  }
}
