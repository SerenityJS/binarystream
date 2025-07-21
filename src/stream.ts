import * as Types from "./types";

class BinaryStream {
  /**
   * The buffer that this stream operates on.
  */
  public readonly buffer: Buffer;

  /**
   * The current offset in the buffer from which to read or write data.
  */
  public offset: number = 0;

  /**
   * Create a new BinaryStream instance.
   * @param buffer The buffer to use for this stream.
   */
  public constructor(buffer: Buffer) {
    this.buffer = buffer;
  }

  /**
   * Read a specified number of bytes from the stream.
   * @note The buffer returned is a subarray of the original buffer.
   * @param length The number of bytes to read.
   * @returns The read bytes as a Buffer.
   */
  public read(length: number): Buffer {
    // Validate the offset before reading
    if (this.offset + length > this.buffer.length) {
      throw new Error('Attempt to read beyond buffer length');
    }

    // Read the bytes from the current offset
    const data = this.buffer.subarray(this.offset, this.offset + length);

    // Increment the offset
    this.offset += length;

    // Return the read bytes
    return data;
  }

  /**
   * Write a Buffer to the stream at the current offset.
   * @param data The Buffer to write to the stream.
   */
  public write(data: Buffer): void {
    // Validate the offset before writing
    if (this.offset + data.length > this.buffer.length) {
      throw new Error('Attempt to write beyond buffer length');
    }

    // Write the data to the current offset
    this.buffer.set(data, this.offset);

    // Increment the offset
    this.offset += data.length;
  }

  /**
   * Validate if the current offset plus the specified length exceeds the buffer length.
   * @param length The length to validate against the current offset.
   * @returns True if the offset is valid, false otherwise.
   */
  public validateOffset(length: number): boolean {
    // Check if the current offset plus the length exceeds the buffer length
    return this.offset + length <= this.buffer.length;
  }

  /**
   * Read a 8-bit unsigned integer (u8) from the stream.
   * @returns The read 8-bit unsigned integer.
   */
  public readUint8(): number {
    return Types.Uint8.read(this);
  }

  /**
   * Write a 8-bit unsigned integer (u8) to the stream.
   * @param value The 8-bit unsigned integer to write.
   */
  public writeUint8(value: number): void {
    Types.Uint8.write(this, value);
  }

  /**
   * Read a 16-bit unsigned integer (u16) from the stream.
   * @returns The read 16-bit unsigned integer.
   */
  public readUint16(): number {
    return Types.Uint16.read(this);
  }

  /**
   * Write a 16-bit unsigned integer (u16) to the stream.
   * @param value The 16-bit unsigned integer to write.
   */
  public writeUint16(value: number): void {
    Types.Uint16.write(this, value);
  }

  /**
   * Read a 24-bit unsigned integer (u24) from the stream.
   * @returns The read 24-bit unsigned integer.
   */
  public readUint24(): number {
    return Types.Uint24.read(this);
  }

  /**
   * Write a 24-bit unsigned integer (u24) to the stream.
   * @param value The 24-bit unsigned integer to write.
   */
  public writeUint24(value: number): void {
    Types.Uint24.write(this, value);
  }

  /**
   * Read a 32-bit unsigned integer (u32) from the stream.
   * @returns The read 32-bit unsigned integer.
   */
  public readUint32(): number {
    return Types.Uint32.read(this);
  }

  /**
   * Write a 32-bit unsigned integer (u32) to the stream.
   * @param value The 32-bit unsigned integer to write.
   */
  public writeUint32(value: number): void {
    Types.Uint32.write(this, value);
  }

  /**
   * Read a 64-bit unsigned integer (u64) from the stream.
   * @returns The read 64-bit unsigned integer.
   */
  public readUint64(): bigint {
    return Types.Uint64.read(this);
  }

  /**
   * Write a 64-bit unsigned integer (u64) to the stream.
   * @param value The 64-bit unsigned integer to write.
   */
  public writeUint64(value: bigint): void {
    Types.Uint64.write(this, value);
  }

  /**
   * Read a variable-length integer (varint) from the stream.
   * @returns The read variable-length integer.
   */
  public readVarInt(): number {
    return Types.VarInt.read(this);
  }

  /**
   * Write a variable-length integer (varint) to the stream.
   * @param value The variable-length integer to write.
   */
  public writeVarInt(value: number): void {
    Types.VarInt.write(this, value);
  }

  /**
   * Read a variable-length long integer (varlong) from the stream.
   * @returns The read variable-length long integer.
   */
  public readVarLong(): bigint {
    return Types.VarLong.read(this);
  }

  /**
   * Write a variable-length long integer (varlong) to the stream.
   * @param value The variable-length long integer to write.
   */
  public writeVarLong(value: bigint): void {
    Types.VarLong.write(this, value);
  }

  /**
   * Read a 8-bit signed integer (i8) from the stream.
   * @returns The read 8-bit signed integer.
   */
  public readInt8(): number {
    return Types.Int8.read(this);
  }

  /**
   * Write a 8-bit signed integer (i8) to the stream.
   * @param value The 8-bit signed integer to write.
   */
  public writeInt8(value: number): void {
    Types.Int8.write(this, value);
  }

  /**
   * Read a 16-bit signed integer (i16) from the stream.
   * @returns The read 16-bit signed integer.
   */
  public readInt16(): number {
    return Types.Int16.read(this);
  }

  /**
   * Write a 16-bit signed integer (i16) to the stream.
   * @param value The 16-bit signed integer to write.
   */
  public writeInt16(value: number): void {
    Types.Int16.write(this, value);
  }

  /**
   * Read a 24-bit signed integer (i24) from the stream.
   * @returns The read 24-bit signed integer.
   */
  public readInt24(): number {
    return Types.Int24.read(this);
  }

  /**
   * Write a 24-bit signed integer (i24) to the stream.
   * @param value The 24-bit signed integer to write.
   */
  public writeInt24(value: number): void {
    Types.Int24.write(this, value);
  }

  /**
   * Read a 32-bit signed integer (i32) from the stream.
   * @returns The read 32-bit signed integer.
   */
  public readInt32(): number {
    return Types.Int32.read(this);
  }

  /**
   * Write a 32-bit signed integer (i32) to the stream.
   * @param value The 32-bit signed integer to write.
   */
  public writeInt32(value: number): void {
    Types.Int32.write(this, value);
  }

  /**
   * Read a 64-bit signed integer (i64) from the stream.
   * @returns The read 64-bit signed integer.
   */
  public readInt64(): bigint {
    return Types.Int64.read(this);
  }

  /**
   * Write a 64-bit signed integer (i64) to the stream.
   * @param value The 64-bit signed integer to write.
   */
  public writeInt64(value: bigint): void {
    Types.Int64.write(this, value);
  }

  /**
   * Read a ZigZag encoded integer from the stream.
   * @returns The decoded integer.
   */
  public readZigZag(): number {
    return Types.ZigZag.read(this);
  }

  /**
   * Write a ZigZag encoded integer to the stream.
   * @param value The integer to encode and write.
   */
  public writeZigZag(value: number): void {
    Types.ZigZag.write(this, value);
  }

  /**
   * Read a ZigZong encoded integer from the stream.
   * @returns The decoded ZigZong integer.
   */
  public readZigZong(): bigint {
    return Types.ZigZong.read(this);
  }

  /**
   * Write a ZigZong encoded integer to the stream.
   * @param value The ZigZong integer to encode and write.
   */
  public writeZigZong(value: bigint): void {
    Types.ZigZong.write(this, value);
  }

  /**
   * Read a 32-bit floating point number from the stream.
   * @returns The read 32-bit float.
   */
  public readFloat32(): number {
    return Types.Float32.read(this);
  }

  /**
   * Write a 32-bit floating point number to the stream.
   * @param value The 32-bit float to write.
   */
  public writeFloat32(value: number): void {
    Types.Float32.write(this, value);
  }

  /**
   * Read a 16-bit string from the stream.
   * @returns The read string.
   */
  public readString16(): string {
    return Types.String16.read(this);
  }

  /**
   * Write a 16-bit string to the stream.
   * @param value The string to write.
   */
  public writeString16(value: string): void {
    Types.String16.write(this, value);
  }

  /**
   * Read a 32-bit string from the stream.
   * @returns The read string.
   */
  public readString32(): string {
    return Types.String32.read(this);
  }

  /**
   * Write a 32-bit string to the stream.
   * @param value The string to write.
   */
  public writeString32(value: string): void {
    Types.String32.write(this, value);
  }

  /**
   * Read a variable-length string from the stream.
   * @returns The read variable-length string.
   */
  public readVarString(): string {
    return Types.VarString.read(this);
  }

  /**
   * Write a variable-length string to the stream.
   * @param value The string to write.
   */
  public writeVarString(value: string): void {
    Types.VarString.write(this, value);
  }

  /**
   * Read a boolean value from the stream.
   * @returns The read boolean value.
   */
  public readBool(): boolean {
    return Types.Bool.read(this);
  }

  /**
   * Write a boolean value to the stream.
   * @param value The boolean value to write.
   */
  public writeBool(value: boolean): void {
    Types.Bool.write(this, value);
  }
}

export { BinaryStream };
