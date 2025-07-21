import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";
import { VarInt } from "../unsigned";

class ZigZag extends DataType {
  /**
   * The size of the ZigZag data type in bytes.
  */
  public static readonly SIZE = 5;

  /**
   * Read an 32-bit signed variable lengthed integer (i32) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): number {
    // Read a variable-length integer from the stream
    let value = VarInt.read(stream);

    // Decode the ZigZag encoding
    value = (value >> 1) ^ (-(value & 1));

    // Return the decoded value
    return value;
  }

  /**
   * Write an 32-bit signed variable lengthed integer (i32) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   */
  public static write(stream: BinaryStream, value: number): void {
    // Encode the ZigZag encoding
    value = (value << 1) ^ (value >> 31);

    // Write the variable-length integer to the stream
    VarInt.write(stream, value);
  }
}

export { ZigZag };
