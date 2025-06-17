use napi::bindgen_prelude::FromNapiValue;
use napi_derive::napi;
use napi::Result;
use crate::binary::BinaryStream;
use crate::stream::Endianness;
use crate::types::Uint16;

#[napi]
#[derive(Clone)]
/**
 * **String**
 * 
 * Represents an unsigned 16-bit variable length ( 2 bytes ) utf-8 string. ( 0 to 65535 )
*/
pub struct String16 {}

#[napi]
impl String16 {
  #[napi]
  /**
   * **read**
   * 
   * Reads an unsigned 16-bit ( 2 bytes ) utf-8 string from the stream. ( 0 to 65535 )
  */
  pub fn read(stream: &mut BinaryStream, endian: Option<Endianness>) -> Result<String> {
    let length = match Uint16::read(stream, endian) {
      Ok(value) => value as u32,
      Err(err) => return Err(err)
    };

    let buffer = match stream.read(length) {
      Ok(bytes) => bytes,
      Err(err) => return Err(err)
    };

    let value = String::from_utf8_lossy(&buffer).to_string();

    Ok(value)
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes an unsigned 16-bit ( 2 bytes ) utf-8 string to the stream. ( 0 to 65535 )
  */
  pub fn write(stream: &mut BinaryStream, value: String, endian: Option<Endianness>) {
    let len = value.len() as u16;
    Uint16::write(stream, len, endian);
    stream.write(value.as_bytes().to_vec())
  }
}

impl FromNapiValue for String16 {
  unsafe fn from_napi_value(_: napi::sys::napi_env, _: napi::sys::napi_value) -> Result<Self> {
    Ok(String16 {})
  }
}