import { Endianness } from "../../endianness";
import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";
import { VarInt } from "../unsigned";

class VarString extends DataType {
  /**
   * The size of the VarString data type in bytes.
  */
  public static readonly SIZE = 5;

  /**
   * Read an 32-bit utf-8 string from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): string {
    // Read the length of the string
    const length = VarInt.read(stream);

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
   * Write an 32-bit utf-8 string to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The string value to write.
   */
  public static write(stream: BinaryStream, value: string): void {
    // Convert the string to a buffer
    const buffer = Buffer.from(value, 'utf8');

    // Validate the length before writing
    if (!stream.validateOffset(buffer.length + VarInt.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the length of the string
    VarInt.write(stream, buffer.length);

    // Write the string bytes to the stream
    stream.write(buffer);
  }
}

export { VarString };
