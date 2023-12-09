use napi_derive::napi;
use crate::binary::BinaryStream;

#[napi]
/**
 * **Bool**
 * 
 * Represents a boolean value. ( true or false )
*/
pub struct Bool {}

#[napi]
impl Bool {
  #[napi]
  /**
   * **read**
   * 
   * Reads a boolean ( 1 byte ) value from the stream. ( true or false )
  */
  pub fn read(stream: &mut BinaryStream) -> bool {
    let bytes = stream.read(1);
    
    return bytes[0] != 0
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a boolean ( 1 byte ) value to the stream. ( true or false )
  */
  pub fn write(stream: &mut BinaryStream, value: bool) {
    let value = match value {
      true => 1,
      false => 0,
    };
    stream.write(vec![value])
  }
}
