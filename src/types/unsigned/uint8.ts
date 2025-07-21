import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";

class Uint8 extends DataType {
  /**
   * The size of the Uint8 data type in bytes.
  */
  public static readonly SIZE = 1;

  /**
   * Read an 8-bit unsigned integer (i8) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): number {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read the byte from the current offset
    const value = stream.buffer.readUint8(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    // Return the unsigned integer value
    return value;
  }

  /**
   * Write an 8-bit unsigned integer (i8) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   */
  public static write(stream: BinaryStream, value: number): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the unsigned integer value as a byte to the stream
    stream.buffer.writeUint8(value, stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Uint8 };
