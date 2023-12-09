use napi_derive::napi;
use crate::binary::BinaryStream;

#[napi]
/**
 * **Byte**
 * 
 * Represents a signed 128-bit ( 16 bytes ) uuid string.
*/
pub struct Uuid {}

#[napi]
impl Uuid {
  #[napi]
  /**
   * **read**
   * 
   * Reads a signed 128-bit ( 16 bytes ) uuid string from the stream.
  */
  pub fn read(stream: &mut BinaryStream) -> String {
    let bytes = stream.read(16);
    let mut uuid = String::new();
    for byte in bytes {
      uuid.push_str(&format!("{:02X}", byte));
    }

    return uuid
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a signed 128-bit ( 16 bytes ) uuid string to the stream.
  */
  pub fn write(stream: &mut BinaryStream, value: String) {
    let mut data = value.replace("-", "");
    data = data.to_uppercase();
    let mut bytes = Vec::new();
    for i in 0..16 {
      let byte = u8::from_str_radix(&data[i * 2..i * 2 + 2], 16).unwrap();
      bytes.push(byte);
    }
    stream.write(bytes)
  }
}
