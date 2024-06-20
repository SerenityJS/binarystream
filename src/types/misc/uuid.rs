use napi_derive::napi;
use napi::Result;
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
  pub fn read(stream: &mut BinaryStream) -> Result<String> {
    // MIGHT NEED TO FORMAT LIKE WRITE?
    // MIGHT ALSO JUST GO BACK TO ORIGINAL?
    let mut bytes_m = match stream.read(8) {
      Ok(bytes_m) => bytes_m,
      Err(err) => return Err(err)
    };

    let mut bytes_l = match stream.read(8) {
      Ok(bytes_l) => bytes_l,
      Err(err) => return Err(err)
    };

    bytes_m.reverse();
    bytes_l.reverse();

    let mut uuid = String::new();

    for byte in bytes_m {
      uuid.push_str(&format!("{:02X}", byte));
    }
    
    for byte in bytes_l {
      uuid.push_str(&format!("{:02X}", byte));
    }

    Ok(uuid)
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
    for i in 0..8 {
      let byte = u8::from_str_radix(&data[i * 2..i * 2 + 2], 16).unwrap();
      bytes.push(byte);
    }

    bytes.reverse();
    stream.write(bytes);
    bytes = Vec::new();

    for i in 8..16 {
      let byte = u8::from_str_radix(&data[i * 2..i * 2 + 2], 16).unwrap();
      bytes.push(byte);
    }

    bytes.reverse();
    stream.write(bytes)
  }
}
