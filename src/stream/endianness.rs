use napi_derive::napi;
use napi::bindgen_prelude::*;

#[napi]
pub enum Endianness {
  Big,
  Little,
}
