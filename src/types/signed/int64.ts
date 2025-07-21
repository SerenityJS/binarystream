import { Endianness } from "../../endianness";
import { BinaryStream } from "../../stream";
import { DataType, DataTypeOptions } from "../data-type";

class Int64 extends DataType {
  /**
   * The size of the Int64 data type in bytes.
  */
  public static readonly SIZE = 8;

  /**
   * Read an 64-bit signed integer (i64) from the stream.
   * @param stream The BinaryStream to read from.
   * @param options Options for reading the data type, such as endianness.
   */
  public static read(stream: BinaryStream, options?: DataTypeOptions): bigint {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read the bytes from the current offset based on endianness
    const value = (options?.endian === undefined || options.endian === Endianness.Big)
      ? stream.buffer.readBigInt64BE(stream.offset)
      : stream.buffer.readBigInt64LE(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    // Return the signed integer value
    return value;
  }

  /**
   * Write an 64-bit signed integer (i64) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The signed integer value to write.
   * @param options Options for writing the data type, such as endianness.
   */
  public static write(stream: BinaryStream, value: bigint, options?: DataTypeOptions): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the signed integer value as a 8-byte integer based on endianness
    if (options?.endian === undefined || options.endian === Endianness.Big) {
      stream.buffer.writeBigInt64BE(value, stream.offset);
    } else {
      stream.buffer.writeBigInt64LE(value, stream.offset);
    }

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Int64 };
