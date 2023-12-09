use napi_derive::napi;
use crate::binary::BinaryStream;

#[napi]
/**
 * **Uint8**
 * 
 * Represents an unsigned 8-bit ( 1 byte ) integer. ( 0 to 255 )
*/
pub struct Uint8 {}

#[napi]
impl Uint8 {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 8-bit ( 1 byte ) integer from the stream. ( 0 to 255 )
  */
  pub fn read(stream: &mut BinaryStream) -> u8 {
    let bytes = stream.read(1);
    
    return bytes[0]
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 8-bit ( 1 byte ) integer to the stream. ( 0 to 255 )
  */
  pub fn write(stream: &mut BinaryStream, value: u8) {
    stream.write(vec![value]);
  }
}
