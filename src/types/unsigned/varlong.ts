import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";

class VarLong extends DataType {
  /**
   * The size of the VarLong data type in bytes.
  */
  public static readonly SIZE = 10;

  /**
   * Read an 64-bit unsigned variable lengthed integer (i64) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): bigint {
    // Prepare the value and size variables
    let value = 0n;
    let size = 0n;

    // Prepare a variable to hold the byte read from the stream
    let byte: bigint;

    // Loop to read bytes until the continuation bit is not set
    do {
      // Read the next byte from the stream
      byte = BigInt(stream.buffer[stream.offset++] || 0n);

      // Shift the value and add the byte
      value |= (byte & 0x7Fn) << (size * 7n);
      size++;

      // Check if we have read too many bytes
      if (size > this.SIZE) throw new Error('VarLong too long');

      // If the continuation bit is not set, we are done
      if ((byte & 0x80n) === 0n) break;
    } while ((byte & 0x80n) !== 0n)

    // Return the decoded value
    return value;
  }

  /**
   * Write an 64-bit unsigned variable lengthed integer (i64) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   */
  public static write(stream: BinaryStream, value: bigint): void {
    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Prepare a variable to hold the byte to write
    let byte: bigint;

    // Loop to write bytes until the value is zero
    do {
      // Get the next byte to write
      byte = value & 0x7Fn;

      // If there are more bits to write, set the continuation bit
      if (value > 0x7Fn) byte |= 0x80n;

      // Write the byte to the stream
      stream.buffer[stream.offset++] = Number(byte);

      // Shift the value right by 7 bits
      value >>= 7n;
    } while (value > 0);
  }
}

export { VarLong };
