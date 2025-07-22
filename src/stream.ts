import { Endianness } from "./endianness";
import * as Types from "./types";

class BinaryStream {

  /**
   * The buffer that this stream operates on.
  */
  public buffer: Buffer;

  /**
   * The current offset in the buffer from which to read or write data.
  */
  public offset: number = 0;

  /**
   * Create a new BinaryStream instance.
   * @param buffer The buffer to use for this stream.
   */
  public constructor(buffer?: Buffer) {
    this.buffer = buffer || Buffer.alloc(0);
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
    // Resize the buffer if necessary
    if (this.offset + length > this.buffer.length) {
      // Create a new buffer with the required size
      const buffer = Buffer.alloc(this.offset + length * 8);

      // Copy the existing data to the new buffer
      buffer.set(this.buffer);

      // Update the stream's buffer to the new buffer
      this.buffer = buffer;
    } else if (this.offset < 0) return false;
    

    // Return true if the offset is valid
    return true;
  }

  /**
   * Get the current buffer of the stream.
   * @returns The current buffer up to the current offset.
   */
  public getBuffer(): Buffer {
    // Return a subarray of the buffer up to the current offset
    return this.buffer.subarray(0, this.offset);
  }

  /**
   * Check if the offset is at the end of the buffer.
   * @returns Whether the offset is at the end.
   */
  public feof(): boolean {
    // Check if the current offset is at the end of the buffer
    return this.offset >= this.buffer.length;
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
   * @param endian The endianness to use for reading.
   * @returns The read 16-bit unsigned integer.
   */
  public readUint16(endian?: Endianness): number {
    return Types.Uint16.read(this, { endian });
  }

  /**
   * Write a 16-bit unsigned integer (u16) to the stream.
   * @param value The 16-bit unsigned integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeUint16(value: number, endian?: Endianness): void {
    Types.Uint16.write(this, value, { endian });
  }

  /**
   * Read a 24-bit unsigned integer (u24) from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read 24-bit unsigned integer.
   */
  public readUint24(endian?: Endianness): number {
    return Types.Uint24.read(this, { endian });
  }

  /**
   * Write a 24-bit unsigned integer (u24) to the stream.
   * @param value The 24-bit unsigned integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeUint24(value: number, endian?: Endianness): void {
    Types.Uint24.write(this, value, { endian });
  }

  /**
   * Read a 32-bit unsigned integer (u32) from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read 32-bit unsigned integer.
   */
  public readUint32(endian?: Endianness): number {
    return Types.Uint32.read(this, {endian });
  }

  /**
   * Write a 32-bit unsigned integer (u32) to the stream.
   * @param value The 32-bit unsigned integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeUint32(value: number, endian?: Endianness): void {
    Types.Uint32.write(this, value, { endian });
  }

  /**
   * Read a 64-bit unsigned integer (u64) from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read 64-bit unsigned integer.
   */
  public readUint64(endian?: Endianness): bigint {
    return Types.Uint64.read(this, { endian });
  }

  /**
   * Write a 64-bit unsigned integer (u64) to the stream.
   * @param value The 64-bit unsigned integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeUint64(value: bigint, endian?: Endianness): void {
    Types.Uint64.write(this, value, { endian });
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
   * @param endian The endianness to use for reading.
   * @returns The read 16-bit signed integer.
   */
  public readInt16(endian?: Endianness): number {
    return Types.Int16.read(this, { endian });
  }

  /**
   * Write a 16-bit signed integer (i16) to the stream.
   * @param value The 16-bit signed integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeInt16(value: number, endian?: Endianness): void {
    Types.Int16.write(this, value, { endian });
  }

  /**
   * Read a 24-bit signed integer (i24) from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read 24-bit signed integer.
   */
  public readInt24(endian?: Endianness): number {
    return Types.Int24.read(this, { endian });
  }

  /**
   * Write a 24-bit signed integer (i24) to the stream.
   * @param value The 24-bit signed integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeInt24(value: number, endian?: Endianness): void {
    Types.Int24.write(this, value, { endian });
  }

  /**
   * Read a 32-bit signed integer (i32) from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read 32-bit signed integer.
   */
  public readInt32(endian?: Endianness): number {
    return Types.Int32.read(this, { endian });
  }

  /**
   * Write a 32-bit signed integer (i32) to the stream.
   * @param value The 32-bit signed integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeInt32(value: number, endian?: Endianness): void {
    Types.Int32.write(this, value, { endian });
  }

  /**
   * Read a 64-bit signed integer (i64) from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read 64-bit signed integer.
   */
  public readInt64(endian?: Endianness): bigint {
    return Types.Int64.read(this, { endian });
  }

  /**
   * Write a 64-bit signed integer (i64) to the stream.
   * @param value The 64-bit signed integer to write.
   * @param endian The endianness to use for writing.
   */
  public writeInt64(value: bigint, endian?: Endianness): void {
    Types.Int64.write(this, value, { endian });
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
   * @param endian The endianness to use for reading.
   * @returns The read 32-bit float.
   */
  public readFloat32(endian?: Endianness): number {
    return Types.Float32.read(this, { endian });
  }

  /**
   * Write a 32-bit floating point number to the stream.
   * @param value The 32-bit float to write.
   * @param endian The endianness to use for writing.
   */
  public writeFloat32(value: number, endian?: Endianness): void {
    Types.Float32.write(this, value, { endian });
  }

  /**
   * Read a 16-bit string from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read string.
   */
  public readString16(endian?: Endianness): string {
    return Types.String16.read(this, { endian });
  }

  /**
   * Write a 16-bit string to the stream.
   * @param value The string to write.
   * @param endian The endianness to use for writing.
   */
  public writeString16(value: string, endian?: Endianness): void {
    Types.String16.write(this, value, { endian });
  }

  /**
   * Read a 32-bit string from the stream.
   * @param endian The endianness to use for reading.
   * @returns The read string.
   */
  public readString32(endian?: Endianness): string {
    return Types.String32.read(this, { endian });
  }

  /**
   * Write a 32-bit string to the stream.
   * @param value The string to write.
   * @param endian The endianness to use for writing.
   */
  public writeString32(value: string, endian?: Endianness): void {
    Types.String32.write(this, value, { endian });
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
