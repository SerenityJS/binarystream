use napi_derive::napi;
use napi::Result;
use napi::bindgen_prelude::*;


#[napi]
pub enum Endianness {
  Big,
  Little,
}

#[napi]
pub struct BinaryStream {
  /**
   * The binary data of the stream.
  */
  pub binary: Vec<u8>,
  /**
   * The current offset of the stream.
   */
  pub offset: u32,
}

#[napi]
impl BinaryStream {
  /**
   * Creates a new BinaryStream with an optional JavaScript Buffer.
  */
  #[napi(constructor)]
  pub fn new(buffer: Option<Buffer>) -> Self {
    let bin = buffer.unwrap_or(Buffer::from(vec![]));
    BinaryStream {
      binary: bin.as_ref().to_vec(),
      offset: 0,
    }
  }

  /**
   * Reads a number of bytes from the stream.
  */
  #[napi]
  pub fn read(&mut self, length: u32) -> Result<Vec<u8>> {
    let start = self.offset as usize;
    let end = (self.offset + length) as usize;
    self.offset += length;
    Ok(self.binary[start..end].to_vec())
  }

  /**
   * Reads a number of bytes from the stream and returns a JavaScript Buffer.
  */
  #[napi]
  pub fn read_buffer(&mut self, length: u32) -> Result<Buffer> {
    let bytes = self.read(length)?;
    Ok(Buffer::from(bytes))
  }

  /**
   * Writes a number of bytes to the stream.
  */
  #[napi]
  pub fn write(&mut self, data: Vec<u8>) -> Result<()> {
    self.binary.extend(data);
    Ok(())
  }

  /**
   * Writes a JavaScript Buffer to the stream.
  */
  #[napi]
  pub fn write_buffer(&mut self, data: Buffer) -> Result<()> {
    self.write(data.as_ref().to_vec())
  }

  /**
   * Reads the remaining bytes from the stream.
  */
  #[napi]
  pub fn read_remaining(&mut self) -> Result<Vec<u8>> {
    let start = self.offset as usize;
    let end = self.binary.len();
    self.offset = end as u32;
    Ok(self.binary[start..end].to_vec())
  }

  /**
   * Reads the remaining bytes from the stream and returns a JavaScript Buffer.
  */
  #[napi]
  pub fn read_remaining_buffer(&mut self) -> Result<Buffer> {
    let bytes = self.read_remaining()?;
    Ok(Buffer::from(bytes))
  }

  /**
   * Skips a number of bytes from the stream.
  */
  #[napi]
  pub fn skip(&mut self, length: u32) -> Result<()> {
    self.offset += length;
    Ok(())
  }

  /**
   * Checks if the cursor is at the end of the stream.
  */
  #[napi]
  pub fn cursor_at_end(&self) -> Result<bool> {
    Ok(self.offset == self.binary.len() as u32)
  }

  /**
   * Checks if the cursor is at the start of the stream.
  */
  #[napi]
  pub fn cursor_at_start(&self) -> Result<bool> {
    Ok(self.offset == 0)
  }

  /**
   * Gets the binary as a JavaScript Buffer.
  */
  #[napi]
  pub fn get_buffer(&self) -> Result<Buffer> {
    Ok(Buffer::from(self.binary.clone()))
  }
}

#[napi]
// String
impl BinaryStream {
  /**
   * Reads a string from the stream.
  */
  #[napi]
  pub fn read_string(&mut self, length: u32) -> Result<String> {
    let bytes = self.read(length)?;
    let string = String::from_utf8(bytes);
    match string {
      Ok(string) => Ok(string),
      Err(_) => Err(Error::new(Status::GenericFailure, "Invalid UTF-8")),
    }
  }

  /**
   * Writes a string to the stream.
  */
  #[napi]
  pub fn write_string(&mut self, data: String) -> Result<()> {
    self.write(data.as_bytes().to_vec())
  }
}

#[napi]
// VarInt BigString
impl BinaryStream {
  #[napi]
  pub fn read_big_string(&mut self) -> Result<String> {
    let length = self.read_var_int()? as usize;
    let value = String::from_utf8_lossy(&&self.binary[self.offset as usize..self.offset as usize + length]).to_string();
    self.offset += length as u32;
    Ok(value)
  }

  #[napi]
  pub fn write_big_string(&mut self, data: String) -> Result<()> {
    let length = data.len() as u32;
    self.write_var_int(length)?;
    self.write_string(data)
  }
}

#[napi]
// LittleString
impl BinaryStream {
  #[napi]
  pub fn read_little_string(&mut self) -> Result<String> {
    let length = self.read_int32(Some(Endianness::Little))? as usize;
    let value = String::from_utf8_lossy(&&self.binary[self.offset as usize..self.offset as usize + length]).to_string();
    self.offset += length as u32;
    Ok(value)
  }

  #[napi]
  pub fn write_little_string(&mut self, data: String) -> Result<()> {
    let length = data.len() as i32;
    self.write_int32(length, Some(Endianness::Little))?;
    self.write_string(data)
  }
}

#[napi]
impl BinaryStream {
  #[napi]
  pub fn read_uuid(&mut self) -> Result<String> {
    let bytes = self.read(16)?;
    let mut uuid = String::new();
    for byte in bytes {
      uuid.push_str(&format!("{:02X}", byte));
    }
    Ok(uuid)
  }

  #[napi]
  pub fn write_uuid(&mut self, data: String) -> Result<()> {
    let mut data = data.replace("-", "");
    data = data.to_uppercase();
    let mut bytes = Vec::new();
    for i in 0..16 {
      let byte = u8::from_str_radix(&data[i * 2..i * 2 + 2], 16).unwrap();
      bytes.push(byte);
    }
    self.write(bytes)
  }
}

#[napi]
// Boolean
impl BinaryStream {
  /**
   * Reads a boolean from the stream.
  */
  #[napi]
  pub fn read_bool(&mut self) -> Result<bool> {
    let bytes = self.read(1)?;
    Ok(bytes[0] != 0)
  }

  /**
   * Writes a boolean to the stream.
  */
  #[napi]
  pub fn write_bool(&mut self, data: bool) -> Result<()> {
    let value = match data {
      true => 1,
      false => 0,
    };
    self.write(vec![value])
  }
}

#[napi]
// Unsigned 8-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 8-bit ( 1 byte ) integer to the stream. ( 0 to 255 )
  */
  #[napi]
  pub fn read_uint8(&mut self) -> Result<u8> {
    let bytes = self.read(1)?;
    Ok(bytes[0])
  }

  /**
   * Writes an unsigned 8-bit ( 1 byte ) integer to the stream. ( 0 to 255 )
  */
  #[napi]
  pub fn write_uint8(&mut self, data: u8) -> Result<()> {
    self.write(vec![data])
  }
}

#[napi]
// Signed 8-bit integer
impl BinaryStream {
  /**
   * Reads a signed 8-bit ( 1 byte ) integer to the stream. ( -128 to 127 )
  */
  #[napi]
  pub fn read_int8(&mut self) -> Result<i8> {
    let bytes = self.read(1)?;
    Ok(bytes[0] as i8)
  }

  /**
   * Writes a signed 8-bit ( 1 byte ) integer to the stream. ( -128 to 127 )
  */
  #[napi]
  pub fn write_int8(&mut self, data: i8) -> Result<()> {
    self.write(vec![data as u8])
  }
}

#[napi]
// Unsigned 16-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 16-bit ( 2 bytes ) integer to the stream. ( 0 to 65535 )
   */
  #[napi]
  pub fn read_uint16(&mut self, endian: Option<Endianness>) -> Result<u16> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(2)?;
    match endian {
      Endianness::Big => Ok(u16::from_be_bytes([bytes[0], bytes[1]])),
      Endianness::Little => Ok(u16::from_le_bytes([bytes[0], bytes[1]])),
    }
  }

  /**
   * Writes an unsigned 16-bit ( 2 bytes ) integer to the stream. ( 0 to 65535 )
  */
  #[napi]
  pub fn write_uint16(&mut self, data: u16, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => self.write(data.to_be_bytes().to_vec()),
      Endianness::Little => self.write(data.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// Signed 16-bit integer
impl BinaryStream {
  /**
   * Reads a signed 16-bit ( 2 bytes ) integer to the stream. ( -32768 to 32767 )
  */
  #[napi]
  pub fn read_int16(&mut self, endian: Option<Endianness>) -> Result<i16> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(2)?;
    match endian {
      Endianness::Big => Ok(i16::from_be_bytes([bytes[0], bytes[1]])),
      Endianness::Little => Ok(i16::from_le_bytes([bytes[0], bytes[1]])),
    }
  }

  /**
   * Writes a signed 16-bit ( 2 bytes ) integer to the stream. ( -32768 to 32767 )
  */
  #[napi]
  pub fn write_int16(&mut self, data: i16, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => self.write(data.to_be_bytes().to_vec()),
      Endianness::Little => self.write(data.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// Unsigned 16-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 16-bit ( 2 bytes ) integer to the stream. ( 0 to 65535 )
  */
  #[napi]
  pub fn read_u_short(&mut self, endian: Option<Endianness>) -> Result<u16> {
    self.read_uint16(endian)
  }

  /**
   * Writes an unsigned 16-bit ( 2 bytes ) integer to the stream. ( 0 to 65535 )
  */
  #[napi]
  pub fn write_u_short(&mut self, data: u16, endian: Option<Endianness>) -> Result<()> {
    self.write_uint16(data, endian)
  }
}

#[napi]
// Signed 16-bit integer
impl BinaryStream {
  /**
   * Reads a signed 16-bit ( 2 bytes ) integer to the stream. ( -32768 to 32767 )
  */
  #[napi]
  pub fn read_short(&mut self, endian: Option<Endianness>) -> Result<i16> {
    self.read_int16(endian)
  }

  /**
   * Writes a signed 16-bit ( 2 bytes ) integer to the stream. ( -32768 to 32767 )
  */
  #[napi]
  pub fn write_short(&mut self, data: i16, endian: Option<Endianness>) -> Result<()> {
    self.write_int16(data, endian)
  }
}

#[napi]
// Unsigned 24-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 24-bit ( 3 bytes ) integer to the stream. ( 0 to 16777215 )
  */
  #[napi]
  pub fn read_uint24(&mut self, endian: Option<Endianness>) -> Result<u32> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(3)?;
    match endian {
      Endianness::Big => Ok(u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])),
      Endianness::Little => Ok(u32::from_le_bytes([0, bytes[0], bytes[1], bytes[2]])),
    }
  }

  /**
   * Writes an unsigned 24-bit ( 3 bytes ) integer to the stream. ( 0 to 16777215 )
  */
  #[napi]
  pub fn write_uint24(&mut self, data: u32, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => self.write(data.to_be_bytes()[1..].to_vec()),
      Endianness::Little => self.write(data.to_le_bytes()[1..].to_vec()),
    }
  }
}

#[napi]
// Signed 24-bit integer
impl BinaryStream {
  /**
   * Reads a signed 24-bit ( 3 bytes ) integer to the stream. ( -8388608 to 8388607 )
  */
  #[napi]
  pub fn read_int24(&mut self, endian: Option<Endianness>) -> Result<i32> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(3)?;
    match endian {
      Endianness::Big => Ok(i32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])),
      Endianness::Little => Ok(i32::from_le_bytes([0, bytes[0], bytes[1], bytes[2]])),
    }
  }

  /**
   * Writes a signed 24-bit ( 3 bytes ) integer to the stream. ( -8388608 to 8388607 )
  */
  #[napi]
  pub fn write_int24(&mut self, data: i32, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => self.write(data.to_be_bytes()[1..].to_vec()),
      Endianness::Little => self.write(data.to_le_bytes()[1..].to_vec()),
    }
  }
}

#[napi]
// Unsigned 32-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 32-bit ( 4 bytes ) integer to the stream. ( 0 to 4294967295 )
  */
  #[napi]
  pub fn read_uint32(&mut self, endian: Option<Endianness>) -> Result<u32> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(4)?;
    match endian {
      Endianness::Big => Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
      Endianness::Little => Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
    }
  }

  /**
   * Writes an unsigned 32-bit ( 4 bytes ) integer to the stream. ( 0 to 4294967295 )
  */
  #[napi]
  pub fn write_uint32(&mut self, data: u32, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => self.write(data.to_be_bytes().to_vec()),
      Endianness::Little => self.write(data.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// Signed 32-bit integer
impl BinaryStream {
  /**
   * Reads a signed 32-bit ( 4 bytes ) integer to the stream. ( -2147483648 to 2147483647 )
  */
  #[napi]
  pub fn read_int32(&mut self, endian: Option<Endianness>) -> Result<i32> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(4)?;
    match endian {
      Endianness::Big => Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
      Endianness::Little => Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
    }
  }

  /**
   * Writes a signed 32-bit ( 4 bytes ) integer to the stream. ( -2147483648 to 2147483647 )
  */
  #[napi]
  pub fn write_int32(&mut self, data: i32, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    match endian {
      Endianness::Big => self.write(data.to_be_bytes().to_vec()),
      Endianness::Little => self.write(data.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// Unsigned 64-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 64-bit ( 8 bytes ) integer to the stream. ( 0 to 18446744073709551615 )
  */
  #[napi]
  pub fn read_uint64(&mut self, endian: Option<Endianness>) -> Result<BigInt> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(8)?;
    match endian {
      Endianness::Big => Ok(BigInt::from(u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))),
      Endianness::Little => Ok(BigInt::from(u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))),
    }
  }

  /**
   * Writes an unsigned 64-bit ( 8 bytes ) integer to the stream. ( 0 to 18446744073709551615 )
  */
  #[napi]
  pub fn write_uint64(&mut self, data: BigInt, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    let value = data.get_u64().1;
    match endian {
      Endianness::Big => self.write(value.to_be_bytes().to_vec()),
      Endianness::Little => self.write(value.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// Signed 64-bit integer
impl BinaryStream {
  /**
   * Reads a signed 64-bit ( 8 bytes ) integer to the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  #[napi]
  pub fn read_int64(&mut self, endian: Option<Endianness>) -> Result<BigInt> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(8)?;
    match endian {
      Endianness::Big => Ok(BigInt::from(i64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))),
      Endianness::Little => Ok(BigInt::from(i64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))),
    }
  }

  /**
   * Writes a signed 64-bit ( 8 bytes ) integer to the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  #[napi]
  pub fn write_int64(&mut self, data: BigInt, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    let value = data.get_i64().0;
    match endian {
      Endianness::Big => self.write(value.to_be_bytes().to_vec()),
      Endianness::Little => self.write(value.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// 32-bit floating point number
impl BinaryStream {
  #[napi]
  pub fn read_float32(&mut self, endian: Option<Endianness>) -> Result<f64> {
    let endian = endian.unwrap_or(Endianness::Big);
    let bytes = self.read(4)?;
    match endian {
      Endianness::Big => Ok(f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64),
      Endianness::Little => Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64),
    }
  }

  #[napi]
  pub fn write_float32(&mut self, data: f64, endian: Option<Endianness>) -> Result<()> {
    let endian = endian.unwrap_or(Endianness::Big);
    let value = data as f32;
    match endian {
      Endianness::Big => self.write(value.to_be_bytes().to_vec()),
      Endianness::Little => self.write(value.to_le_bytes().to_vec()),
    }
  }
}

#[napi]
// Unsigned 64-bit integer
impl BinaryStream {
  /**
   * Reads an unsigned 64-bit ( 8 bytes ) integer to the stream. ( 0 to 18446744073709551615 )
  */
  #[napi]
  pub fn read_u_long(&mut self, endian: Option<Endianness>) -> Result<BigInt> {
    self.read_uint64(endian)
  }

  /**
   * Writes an unsigned 64-bit ( 8 bytes ) integer to the stream. ( 0 to 18446744073709551615 )
  */
  #[napi]
  pub fn write_u_long(&mut self, data: BigInt, endian: Option<Endianness>) -> Result<()> {
    self.write_uint64(data, endian)
  }
}

#[napi]
// Signed 64-bit integer
impl BinaryStream {
  /**
   * Reads a signed 64-bit ( 8 bytes ) integer to the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  #[napi]
  pub fn read_long(&mut self, endian: Option<Endianness>) -> Result<BigInt> {
    self.read_int64(endian)
  }

  /**
   * Writes a signed 64-bit ( 8 bytes ) integer to the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  #[napi]
  pub fn write_long(&mut self, data: BigInt, endian: Option<Endianness>) -> Result<()> {
    self.write_int64(data, endian)
  }
}

#[napi]
// 32 bit usigned varianble length integer
impl BinaryStream {
  /**
   * Reads a 32 bit ( 4 bytes ) unsigned variable length integer from the stream. ( 0 to 4294967295 )
  */
  #[napi]
  pub fn read_var_int(&mut self) -> Result<u32> {
    let mut value = 0;
    let mut size = 0;
    loop {
      let byte = self.read_uint8()?;
      value |= (byte as u32 & 0x7F) << (size * 7);
      size += 1;
      if size > 5 {
        return Err(Error::new(Status::GenericFailure, "VarInt is too big"));
      }
      if (byte & 0x80) != 0x80 {
        break;
      }
    }
    Ok(value)
  }

  /**
   * Writes a 32 bit ( 4 bytes ) unsigned variable length integer to the stream. ( 0 to 4294967295 )
  */
  #[napi]
  pub fn write_var_int(&mut self, data: u32) -> Result<()> {
    let mut value = data;
    loop {
      let mut byte = (value & 0x7F) as u8;
      value >>= 7;
      if value != 0 {
        byte |= 0x80;
      }
      self.write_uint8(byte)?;
      if value == 0 {
        break;
      }
    }
    Ok(())
  }
}

#[napi]
// 32 bit signed varianble length integer
impl BinaryStream {
  /**
   * Reads a 32 bit ( 4 bytes ) zigzag encoded signed variable length integer from the stream. ( -2147483648 to 2147483647 )
  */
  #[napi]
  pub fn read_zig_zag(&mut self) -> Result<i32> {
    let value = self.read_var_int()?;
    Ok(((value >> 1) as i32) ^ (-((value & 1) as i32)))
  }

  /**
   * Writes a 32 bit ( 4 bytes ) zigzag encoded signed variable length integer to the stream. ( -2147483648 to 2147483647 )
  */
  #[napi]
  pub fn write_zig_zag(&mut self, data: i32) -> Result<()> {
    let value = ((data << 1) ^ (data >> 31)) as u32;
    self.write_var_int(value)
  }
}

#[napi]
// 64 bit usigned varianble length integer
impl BinaryStream {
  /**
   * Reads a 64 bit ( 8 bytes ) unsigned variable length integer from the stream. ( 0 to 18446744073709551615 )
  */
  #[napi]
  pub fn read_var_long(&mut self) -> Result<BigInt> {
    let mut num_read = 0;
    let mut result = 0;

    loop {
      let read = self.read_uint8()? as u64;
      let value = read & 0b01111111;
      result |= value << (7 * num_read);
      num_read += 1;
      if num_read > 10 {
        return Err(Error::new(Status::GenericFailure, "VarLong is too big"));
      }

      if (read & 0b10000000) == 0 {
        break;
      }
    }

    Ok(BigInt::from(result))
  }

  /**
   * Writes a 64 bit ( 8 bytes ) unsigned variable length integer to the stream. ( 0 to 18446744073709551615 )
  */
  #[napi]
  pub fn write_var_long(&mut self, data: BigInt) -> Result<()> {
    let mut value = data.get_u64().1;
    loop {
      let mut temp = (value & 0b01111111) as u8;
      value >>= 7;
      if value != 0 {
        temp |= 0b10000000;
      }
      self.write_uint8(temp)?;
      if value == 0 {
        break;
      }
    }
    Ok(())
  }
}

#[napi]
// 64 bit signed varianble length integer
impl BinaryStream {
  /**
   * Reads a 64 bit ( 8 bytes ) zigzag encoded signed variable length integer from the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  #[napi]
  pub fn read_zig_zong(&mut self) -> Result<BigInt> {
    let value = self.read_var_long()?;
    let value = value.get_u64().1;

    let value = ((value >> 1) as i64) ^ (-((value & 1) as i64));
    let signed = value < 0;

    let value = match signed {
      true => (-value) as u64,
      false => value as u64,
    };
    
    let value = napi::bindgen_prelude::BigInt {
      sign_bit: signed,
      words: vec![value],
    };

    Ok(value)
  }

  /**
   * Writes a 64 bit ( 8 bytes ) zigzag encoded signed variable length integer to the stream. ( -9223372036854775808 to 9223372036854775807 )
  */
  #[napi]
  pub fn write_zig_zong(&mut self, data: BigInt) -> Result<()> {
    let value = data.get_i64().0;
    let value = ((value << 1) ^ (value >> 63)) as u64;
    let value = napi::bindgen_prelude::BigInt {
      sign_bit: false,
      words: vec![value],
    };
    self.write_var_long(value)
  }
}