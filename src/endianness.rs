use napi_derive::napi;

#[napi]
pub enum Endianness {
  Big = 0,
  Little = 1,
}
