use napi_derive::napi;
use crate::binary::{ BinaryStream, Endianness };

#[napi]
/**
 * **Uint16**
 * 
 * Represents an unsigned 16-bit ( 2 bytes ) integer. ( 0 to 65535 )
*/
pub struct Uint16 {}

#[napi]
impl Uint16 {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 16-bit ( 2 bytes ) integer from the stream. ( 0 to 65535 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> u16 {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(2);
    match endian {
      Endianness::Big => return u16::from_be_bytes([bytes[0], bytes[1]]),
      Endianness::Little => return u16::from_le_bytes([bytes[0], bytes[1]]),
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 16-bit ( 2 bytes ) integer to the stream. ( 0 to 65535 )
  */
  pub fn write(stream: &mut BinaryStream, value: u16, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes().to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes().to_vec()),
    }
  }
}
