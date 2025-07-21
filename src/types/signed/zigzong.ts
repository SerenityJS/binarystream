import { BinaryStream } from "../../stream";
import { DataType } from "../data-type";
import { VarLong } from "../unsigned";

class ZigZong extends DataType {
  /**
   * The size of the ZigZong data type in bytes.
  */
  public static readonly SIZE = 10;

  /**
   * Read an 32-bit signed variable lengthed integer (i32) from the stream.
   * @param stream The BinaryStream to read from.
   */
  public static read(stream: BinaryStream): bigint {
    // Read a variable-length integer from the stream
    let value = VarLong.read(stream);

    // Decode the ZigZong encoding
    value = (value >> 1n) ^ (-(value & 1n));

    // Return the decoded value
    return value;
  }

  /**
   * Write an 32-bit signed variable lengthed integer (i32) to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The unsigned integer value to write.
   */
  public static write(stream: BinaryStream, value: bigint): void {
    // Encode the ZigZong encoding
    value = (value << 1n) ^ (value >> 63n);

    // Write the variable-length integer to the stream
    VarLong.write(stream, value);
  }
}

export { ZigZong };
