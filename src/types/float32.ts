import { Endianness } from "../endianness";
import { BinaryStream } from "../stream";
import { DataType, DataTypeOptions } from "./data-type";

class Float32 extends DataType {
  /**
   * The size of the Float32 data type in bytes.
  */
  public static readonly SIZE = 4;

  /**
   * Read an 32-bit floating-point number (float32) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream, options?: DataTypeOptions): number {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read the bytes from the current offset based on endianness
    const value = (options?.endian === undefined || options.endian === Endianness.Big)
      ? stream.buffer.readFloatBE(stream.offset)
      : stream.buffer.readFloatLE(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    // Return the signed integer value
    return value;
  }

  /**
   * Write an 32-bit floating-point number (float32) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The signed integer value to write.
   */
  public static write(stream: BinaryStream, value: number, options?: DataTypeOptions): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the signed integer value as a 4-byte integer based on endianness
    if (options?.endian === undefined || options.endian === Endianness.Big) {
      stream.buffer.writeFloatBE(value, stream.offset);
    } else {
      stream.buffer.writeFloatLE(value, stream.offset);
    }

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Float32 };
