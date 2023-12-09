use napi_derive::napi;
use crate::{binary::BinaryStream, stream::Endianness};

#[napi]
/**
 * **Float32**
 * 
 * Respresents a signed 32-bit ( 4 bytes ) floating point number. ( -3.402823e38 to 3.402823e38 )
*/
pub struct Float32 {}

#[napi]
impl Float32 {
  #[napi]
  /**
   * **read**
   * 
   * Reads a signed 32-bit ( 4 bytes ) floating point number from the stream. ( -3.402823e38 to 3.402823e38 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> f64 {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = stream.read(4);
    match endian {
      Endianness::Big => return f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64,
      Endianness::Little => return f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64,
    }
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a signed 32-bit ( 4 bytes ) floating point number to the stream. ( -3.402823e38 to 3.402823e38 )
  */
  pub fn write(stream: &mut BinaryStream, value: f64, endian: Option<Endianness>) {
    let endian = endian.unwrap_or(Endianness::Big);
    let value = value as f32;
    match endian {
      Endianness::Big => stream.write(value.to_be_bytes().to_vec()),
      Endianness::Little => stream.write(value.to_le_bytes().to_vec()),
    }
  }
}
