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
    let value = 0n;

    // Iterate through the stream to read bytes until we reach the end of the VarInt
    for (let i = 0; i < this.SIZE; i++) {
      // Read the next byte from the stream
      let byte = stream.buffer[stream.offset++] || 0;

      // Shift the value and add the byte
      value |= (BigInt(byte) & 0x7Fn) << (BigInt(i) * 7n);

      // Check if the continuation bit is not set, we are done
      if ((byte & 0x80) === 0) return Number(value)
    }

    // Throw an error if we exceed the maximum size
    throw new Error('VarInt exceeds maximum size');
  }

  /**
   * Write an 32-bit unsigned variable lengthed integer (i32) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   */
  public static write(stream: BinaryStream, value: number): void {
    // Ensure the value is unsigned
    value = value >>> 0;

    // Validate the offset before writing
    if (!stream.validateOffset(this.SIZE)) {
      throw new Error('Write exceeds buffer length');
    }

    // Iterate through the maximum size of the VarInt
    for (let i = 0; i < this.SIZE; i++) {
      // Check if the value is still greater than 0x7F
      if (value >> 7 !== 0) {
        // Write the byte with the continuation bit set
        stream.buffer[stream.offset++] = (value & 0x7F) | 0x80;
      } else {
        // Write the last byte without the continuation bit
        stream.buffer[stream.offset++] = value & 0x7F;
        return; // Return as we are done
      }

      // Shift the value right by 7 bits for the next iteration
      value >>>= 7;
    }
  }
}

export { VarInt };
