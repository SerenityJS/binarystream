use napi_derive::napi;

#[napi]
/**
 * **DataType**
 * 
 * Represents an abstract data type within JavaScript.
*/
pub struct DataType {}

#[napi]
impl DataType {
  #[napi(ts_args_type = "stream: BinaryStream", ts_return_type="any")]
  pub fn read() {
    println!("DataType.read() is not implemented!");
  }

  #[napi(ts_args_type = "stream: BinaryStream, value: any")]
  pub fn write() {
    println!("DataType.write() is not implemented!");
  }
}
