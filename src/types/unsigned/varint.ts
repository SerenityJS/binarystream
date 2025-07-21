import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";

class VarInt extends DataType {
  /**
   * The size of the VarInt data type in bytes.
  */
  public static readonly SIZE = 5;

  /**
   * Read an 32-bit unsigned variable lengthed integer (i32) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): number {
    // Prepare the value and size variables
    let value = 0;
    let size = 0;

    // Prepare a variable to hold the byte read from the stream
    let byte: number;

    // Loop to read bytes until the continuation bit is not set
    do {
      // Read the next byte from the stream
      byte = stream.buffer[stream.offset++] || 0;

      // Shift the value and add the byte
      value |= (byte & 0x7F) << (size * 7);
      size++;

      // Check if we have read too many bytes
      if (size > this.SIZE) throw new Error('VarInt too long');

      // If the continuation bit is not set, we are done
      if ((byte & 0x80) === 0) break;
    } while ((byte & 0x80) !== 0)

    // Return the decoded value
    return value;
  }

  /**
   * Write an 32-bit unsigned variable lengthed integer (i32) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   */
  public static write(stream: BinaryStream, value: number): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Prepare a variable to hold the byte to write
    let byte: number;

    // Loop to write bytes until the value is zero
    do {
      // Get the next byte to write
      byte = value & 0x7F;

      // If there are more bits to write, set the continuation bit
      if (value > 0x7F) byte |= 0x80;

      // Write the byte to the stream
      stream.buffer[stream.offset++] = byte;

      // Shift the value right by 7 bits
      value >>= 7;
    } while (value > 0);
  }
}

export { VarInt };
