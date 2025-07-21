import { Endianness } from "../../endianness";
import { BinaryStream } from "../../stream";
import { DataType, DataTypeOptions } from "../data-type";

class Uint24 extends DataType {
  /**
   * The size of the Uint24 data type in bytes.
  */
  public static readonly SIZE = 3;

  /**
   * Read an 24-bit unsigned integer (i24) from the stream.
   * @param stream The BinaryStream to read from.
   * @param options Options for reading the data type, such as endianness.
   */
  public static read(stream: BinaryStream, options?: DataTypeOptions): number {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read the bytes from the current offset based on endianness
    const byte1 = stream.buffer.readUint8(stream.offset++);
    const byte2 = stream.buffer.readUint8(stream.offset++);
    const byte3 = stream.buffer.readUint8(stream.offset++);

    // Combine the bytes into a 24-bit unsigned integer
    const value = (options?.endian === undefined || options.endian === Endianness.Big)
      ? (byte1 << 16) | (byte2 << 8) | byte3
      : (byte3 << 16) | (byte2 << 8) | byte1;

    // Return the unsigned integer value
    return value;
  }

  /**
   * Write an 24-bit unsigned integer (i24) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   * @param options Options for writing the data type, such as endianness.
   */
  public static write(stream: BinaryStream, value: number, options?: DataTypeOptions): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the unsigned integer value as a 3-byte integer based on endianness
    const byte1 = (value >> 16) & 0xFF;
    const byte2 = (value >> 8) & 0xFF;
    const byte3 = value & 0xFF;

    // Write the bytes to the current offset based on endianness
    if (options?.endian === undefined || options.endian === Endianness.Big) {
      stream.buffer.writeUint8(byte1, stream.offset++);
      stream.buffer.writeUint8(byte2, stream.offset++);
      stream.buffer.writeUint8(byte3, stream.offset++);
    } else {
      stream.buffer.writeUint8(byte3, stream.offset++);
      stream.buffer.writeUint8(byte2, stream.offset++);
      stream.buffer.writeUint8(byte1, stream.offset++);
    }
  }
}

export { Uint24 };
