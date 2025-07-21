import { Endianness } from "../../endianness";
import { BinaryStream } from "../../stream";
import { DataType, DataTypeOptions } from "../data-type";
import { Uint16 } from "../unsigned";

class String16 extends DataType {
  /**
   * The size of the String16 data type in bytes.
  */
  public static readonly SIZE = 2;

  /**
   * Read an 16-bit utf-8 string from the stream.
   * @param stream The BinaryStream to read from.
   * @param options Options for reading the data type, such as endianness.
   */
  public static read(stream: BinaryStream, options?: DataTypeOptions): string {
    // Read the length of the string
    const length = Uint16.read(stream, options);

    // Validate the length
    if (!stream.validateOffset(length)) {
      throw new Error('String length exceeds buffer length');
    }

    // Read the string bytes from the current offset based on endianness
    const buffer = stream.read(length);

    // Convert the buffer to a string
    const value = buffer.toString('utf8');

    // Return the string value
    return value;
  }

  /**
   * Write an 16-bit utf-8 string to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The string value to write.
   * @param options Options for writing the data type, such as endianness.
   */
  public static write(stream: BinaryStream, value: string, options?: DataTypeOptions): void {
    // Convert the string to a buffer
    const buffer = Buffer.from(value, 'utf8');

    // Validate the length before writing
    if (!stream.validateOffset(buffer.length + Uint16.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the length of the string
    Uint16.write(stream, buffer.length, options);

    // Write the string bytes to the stream
    stream.write(buffer);
  }
}

export { String16 };
