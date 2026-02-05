import { Endianness } from "../endianness";
import { BinaryStream } from "../stream";
import { DataType, DataTypeOptions } from "./data-type";

class Float64 extends DataType {
  /**
   * The size of the Float64 data type in bytes.
   */
  public static readonly SIZE = 8;

  /**
   * Read a 64-bit floating-point number (float64) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream, options?: DataTypeOptions): number {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error("Read exceeds buffer length");
    }

    const value =
      options?.endian === undefined || options.endian === Endianness.Big
        ? stream.buffer.readDoubleBE(stream.offset)
        : stream.buffer.readDoubleLE(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    return value;
  }

  /**
   * Write a 64-bit floating-point number (float64) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The floating-point value to write.
   */
  public static write(
    stream: BinaryStream,
    value: number,
    options?: DataTypeOptions
  ): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error("Write exceeds buffer length");
    }

    if (options?.endian === undefined || options.endian === Endianness.Big) {
      stream.buffer.writeDoubleBE(value, stream.offset);
    } else {
      stream.buffer.writeDoubleLE(value, stream.offset);
    }

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Float64 };
