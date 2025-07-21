import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";

class Int8 extends DataType {
  /**
   * The size of the Int8 data type in bytes.
  */
  public static readonly SIZE = 1;

  /**
   * Read an 8-bit signed integer (i8) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): number {
    // Validate the offset before reading
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Read exceeds buffer length');
    }

    // Read the byte from the current offset
    const value = stream.buffer.readInt8(stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;

    // Return the signed integer value
    return value;
  }

  /**
   * Write an 8-bit signed integer (i8) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The signed integer value to write.
   */
  public static write(stream: BinaryStream, value: number): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Write the signed integer value as a byte to the stream
    stream.buffer.writeInt8(value, stream.offset);

    // Increment the offset
    stream.offset += this.SIZE;
  }
}

export { Int8 };
