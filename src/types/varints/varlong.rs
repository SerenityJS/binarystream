use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use crate::binary::BinaryStream;
use crate::types::Uint8;

#[napi]
/**
 * **VarLong**
 * 
 * Represents a 64 bit ( 8 bytes ) unsigned variable length integer. ( 0 to 18446744073709551615 )
*/
pub struct VarLong {}

#[napi]
impl VarLong {
  #[napi]
  /**
   * **read**
   * 
   * Reads a 64 bit ( 8 bytes ) unsigned variable length integer from the stream. ( 0 to 18446744073709551615 )
  */
  pub fn read(stream: &mut BinaryStream) -> BigInt {
    let mut num_read = 0;
    let mut result = 0;

    loop {
      let read = Uint8::read(stream) as u64;
      let value = read & 0b01111111;
      result |= value << (7 * num_read);
      num_read += 1;
      if num_read > 10 {
        println!("VarLong is too big");
        return BigInt::from(0 as u64);
      }

      if (read & 0b10000000) == 0 {
        break;
      }
    }

    BigInt::from(result)
  }

  #[napi]
  /**
   * **write**
   * 
   * Writes a 64 bit ( 8 bytes ) unsigned variable length integer to the stream. ( 0 to 18446744073709551615 )
  */
  pub fn write(stream: &mut BinaryStream, value: BigInt) {
    let mut value = value.get_u64().1;
    loop {
      let mut byte = (value & 0b01111111) as u8;
      value >>= 7;
      if value != 0 {
        byte |= 0b10000000;
      }
      Uint8::write(stream, byte);
      if value == 0 {
        break;
      }
    }
  }
}
