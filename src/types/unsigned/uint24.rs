use napi_derive::napi;
use crate::binary::{ BinaryStream, Endianness };

#[napi]
/**
 * **Uint24**
 * 
 * Represents an unsigned 24-bit ( 3 bytes ) integer. ( 0 to 16777215 )
*/
pub struct Uint24 {}

#[napi]
impl Uint24 {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 24-bit ( 3 bytes ) integer from the stream. ( 0 to 16777215 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> u32 {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(3);
    match endian {
      Endianness::Big => return u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]),
      Endianness::Little => return u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]),
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 24-bit ( 3 bytes ) integer to the stream. ( 0 to 16777215 )
  */
  pub fn write(stream: &mut BinaryStream, value: u32, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes()[1..].to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes()[..3].to_vec()),
    }
  }
}
