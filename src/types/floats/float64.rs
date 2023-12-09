use napi_derive::napi;
use crate::{binary::BinaryStream, stream::Endianness};

#[napi]
/**
 * **Float64**
 * 
 * Respresents a signed 64 bit ( 8 bytes ) floating point number. ( -1.7976931348623157e308 to 1.7976931348623157e308 )
*/
pub struct Float64 {}

#[napi]
impl Float64 {
  #[napi]
  /**
   * **read**
   * 
   * Reads a signed 64 bit ( 8 bytes ) floating point number from the stream. ( -1.7976931348623157e308 to 1.7976931348623157e308 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> f64 {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(8);
    match endian {
      Endianness::Big => return f64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]),
      Endianness::Little => return f64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]),
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a signed 64 bit ( 8 bytes ) floating point number to the stream. ( -1.7976931348623157e308 to 1.7976931348623157e308 )
  */
  pub fn write(stream: &mut BinaryStream, value: f64, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes().to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes().to_vec()),
    }
  }
}
