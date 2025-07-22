import { Endianness } from "../endianness";
import { BinaryStream } from "../stream";

interface DataTypeOptions {
  /**
   * The endianness to use for reading or writing data.
   */
  endian?: Endianness;
}

class DataType {
  /**
   * Default constructor for DataType.
   */
  public constructor(..._: Array<unknown>) {
    return;
  }

  /**
   * Read a value from the stream.
   * @param stream The BinaryStream to read from.
   * @param options Options for reading the data type, such as endianness.
   * @returns The value read from the stream.
   */
  public static read(
    _stream: BinaryStream,
    _options?: DataTypeOptions
  ): unknown {
    return;
  }

  /**
   * Write a value to the stream.
   * @param stream The BinaryStream to write to.
   * @param value The value to write.
   * @param options Options for writing the data type, such as endianness.
   */
  public static write(
    _stream: BinaryStream,
    _value: unknown,
    _options?: DataTypeOptions
  ): void {
    return;
  }
}

export { DataType, DataTypeOptions };
