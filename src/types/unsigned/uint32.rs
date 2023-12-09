use napi_derive::napi;
use crate::binary::{ BinaryStream, Endianness };

#[napi]
/**
 * **Uint32**
 * 
 * Represents an unsigned 32-bit ( 4 bytes ) integer. ( 0 to 4294967295 )
*/
pub struct Uint32 {}

#[napi]
impl Uint32 {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 32-bit ( 4 bytes ) integer from the stream. ( 0 to 4294967295 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> u32 {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(4);
    match endian {
      Endianness::Big => return u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
      Endianness::Little => return u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 32-bit ( 4 bytes ) integer to the stream. ( 0 to 4294967295 )
  */
  pub fn write(stream: &mut BinaryStream, value: u32, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes().to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes().to_vec()),
    }
  }
}
