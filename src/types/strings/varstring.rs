use napi::bindgen_prelude::FromNapiValue;
use napi_derive::napi;
use napi::Result;
use crate::binary::BinaryStream;
use crate::types::VarInt;

#[napi]
#[derive(Clone)]
/**
 * **String**
 * 
 * Represents a signed 32-bit variable length ( 4 bytes ) utf-8 string. ( 0 to 4294967295 )
*/
pub struct VarString {}

#[napi]
impl VarString {
  #[napi]
  /**
   * **read**
   * 
   * Reads a signed 32-bit ( 4 bytes ) utf-8 string from the stream. ( 0 to 4294967295 )
  */
  pub fn read(stream: &mut BinaryStream) -> Result<String> {
    // Read the length of the string.
    let length = match VarInt::read(stream) {
      Ok(value) => value as usize,
      Err(err) => return Err(err)
    };

    // Length validation
    let start = stream.offset as usize;
    let end = start + length as usize;
    if end > stream.binary.len() {
      return Err(
        napi::Error::new(
          napi::Status::GenericFailure,
          "String length exceeds available bytes in the stream.".to_string()
        )
      );
    }

    // Read the string from the binary stream.
    let value = String::from_utf8_lossy(&stream.binary[start..end]).to_string();
    stream.offset += length as u32;
    
    Ok(value)
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a signed 32-bit ( 4 bytes ) utf-8 string to the stream. ( 0 to 4294967295 )
  */
  pub fn write(stream: &mut BinaryStream, value: String) {
    let length = value.len() as u32;
    let vec = value.as_bytes().to_vec();
    VarInt::write(stream, length);
    stream.write(vec)
  }
}

impl FromNapiValue for VarString {
  unsafe fn from_napi_value(_: napi::sys::napi_env, _: napi::sys::napi_value) -> Result<Self> {
    Ok(VarString {})
  }
}