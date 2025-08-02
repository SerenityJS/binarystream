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

    // Iterate through the stream to read bytes until we reach the end of the VarInt
    for (let i = 0; i < this.SIZE; i++) {
      // Read the next byte from the stream
      let byte = stream.buffer[stream.offset++] || 0;

      // Shift the value and add the byte
      value |= (BigInt(byte) & 0x7Fn) << (BigInt(i) * 7n);

      // Check if the continuation bit is not set, we are done
      if ((byte & 0x80) === 0) break;
    }

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

    // Iterate through the maximum size of the VarInt
    for (let i = 0; i < this.SIZE; i++) {
      // Check if the value is still greater than 0x7F
      if (value >> 7n !== 0n) {
        // Write the byte with the continuation bit set
        stream.buffer[stream.offset++] = Number((value & 0x7Fn) | 0x80n);
      } else {
        // Write the last byte without the continuation bit
        stream.buffer[stream.offset++] = Number(value & 0x7Fn);
        break; // Break the loop as we are done
      }

      // Shift the value right by 7 bits for the next iteration
      value >>= 7n;
    }
  }
}

export { VarLong };
