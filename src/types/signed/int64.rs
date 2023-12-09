use napi_derive::napi;
use napi::bindgen_prelude::BigInt;
use crate::binary::{ BinaryStream, Endianness };

#[napi]
/**
 * **Int64**
 * 
 * Represents an unsigned 64-bit ( 8 bytes ) integer. ( 0 to 18446744073709551615 )
*/
pub struct Int64 {}

#[napi]
impl Int64 {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 64-bit ( 8 bytes ) integer from the stream. ( 0 to 18446744073709551615 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> BigInt {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(8);
    match endian {
      Endianness::Big => return BigInt::from(i64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])),
      Endianness::Little => return BigInt::from(i64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])),
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 64-bit ( 8 bytes ) integer to the stream. ( 0 to 18446744073709551615 )
  */
  pub fn write(stream: &mut BinaryStream, value: BigInt, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    let value = value.get_i64().0;
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes().to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes().to_vec()),
    }
  }
}
