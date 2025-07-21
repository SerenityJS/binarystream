import { BinaryStream } from "../stream";
import { DataType } from "./data-type";

class Bool extends DataType {
  /**
   * The size of the Bool data type in bytes.
  */
  public static readonly SIZE = 1;

  /**
   * Read a boolean value from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): boolean {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read the byte from the current offset
    const value = stream.buffer.readInt8(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    // Return the boolean value (0 is false, non-zero is true)
    return value !== 0;
  }

  /**
   * Write a boolean value to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The signed integer value to write.
   */
  public static write(stream: BinaryStream, value: boolean): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the boolean value as a byte to the stream
    stream.buffer.writeInt8(value ? 1 : 0, stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Bool };
