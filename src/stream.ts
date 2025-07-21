class BinaryStream {
  /**
   * The buffer that this stream operates on.
  */
  public readonly buffer: Buffer;

  /**
   * The current offset in the buffer from which to read or write data.
  */
  public offset: number = 0;

  /**
   * Create a new BinaryStream instance.
   * @param buffer The buffer to use for this stream.
   */
  public constructor(buffer: Buffer) {
    this.buffer = buffer;
  }

  /**
   * Read a specified number of bytes from the stream.
   * @note The buffer returned is a subarray of the original buffer.
   * @param length The number of bytes to read.
   * @returns The read bytes as a Buffer.
   */
  public read(length: number): Buffer {
    // Validate the offset before reading
    if (this.offset + length > this.buffer.length) {
      throw new Error('Attempt to read beyond buffer length');
    }

    // Read the bytes from the current offset
    const data = this.buffer.subarray(this.offset, this.offset + length);

    // Increment the offset
    this.offset += length;

    // Return the read bytes
    return data;
  }

  /**
   * Write a Buffer to the stream at the current offset.
   * @param data The Buffer to write to the stream.
   */
  public write(data: Buffer): void {
    // Validate the offset before writing
    if (this.offset + data.length > this.buffer.length) {
      throw new Error('Attempt to write beyond buffer length');
    }

    // Write the data to the current offset
    this.buffer.set(data, this.offset);

    // Increment the offset
    this.offset += data.length;
  }

  /**
   * Validate if the current offset plus the specified length exceeds the buffer length.
   * @param length The length to validate against the current offset.
   * @returns True if the offset is valid, false otherwise.
   */
  public validateOffset(length: number): boolean {
    // Check if the current offset plus the length exceeds the buffer length
    return this.offset + length <= this.buffer.length;
  }
}

export { BinaryStream };
