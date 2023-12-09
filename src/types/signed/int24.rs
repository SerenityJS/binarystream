use napi_derive::napi;
use crate::binary::{ BinaryStream, Endianness };

#[napi]
/**
 * **Int24**
 * 
 * Represents a signed 24-bit ( 3 bytes ) integer. ( -8388608 to 8388607 )
*/
pub struct Int24 {}

#[napi]
impl Int24 {
  #[napi]
  /**
   * **read**
   * 
   * Reads a signed 24-bit ( 3 bytes ) integer from the stream. ( -8388608 to 8388607 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> i32 {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(3);
    match endian {
      Endianness::Big => return i32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]),
      Endianness::Little => return i32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]),
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a signed 24-bit ( 3 bytes ) integer to the stream. ( -8388608 to 8388607 )
  */
  pub fn write(stream: &mut BinaryStream, value: i32, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes()[1..].to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes()[..3].to_vec()),
    }
  }
}
