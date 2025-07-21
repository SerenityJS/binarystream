import { Endianness } from "../../endianness";
import { BinaryStream } from "../../stream";
import { DataType, DataTypeOptions } from "../data-type";

class Uint16 extends DataType {
  /**
   * The size of the Uint16 data type in bytes.
  */
  public static readonly SIZE = 2;

  /**
   * Read an 16-bit unsigned integer (i16) from the stream.
   * @param stream The BinaryStream to read from.
   * @param options Options for reading the data type, such as endianness.
   */
  public static read(stream: BinaryStream, options?: DataTypeOptions): number {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read 2 bytes from the current offset based on endianness
    const value = (options?.endian === undefined || options.endian === Endianness.Big)
      ? stream.buffer.readUint16BE(stream.offset)
      : stream.buffer.readUint16LE(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    // Return the unsigned integer value
    return value;
  }

  /**
   * Write an 16-bit unsigned integer (i16) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   * @param options Options for writing the data type, such as endianness.
   */
  public static write(stream: BinaryStream, value: number, options?: DataTypeOptions): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the unsigned integer value as a 2-byte integer based on endianness
    if (options?.endian === undefined || options.endian === Endianness.Big) {
      stream.buffer.writeUint16BE(value, stream.offset);
    } else {
      stream.buffer.writeUint16LE(value, stream.offset);
    }

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Uint16 };
