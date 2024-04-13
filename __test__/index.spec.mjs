import test from 'ava'

import { BinaryStream } from "../index.js"

const stream = new BinaryStream();

test('Int8', (t) => {
  const expected = 127

  stream.writeInt8(expected)

  t.is(stream.readInt8(), expected)
})

test('Uint8', (t) => {
  const expected = 255

  stream.writeUint8(expected)

  t.is(stream.readUint8(), expected)
})

test('Int16', (t) => {
  const expected = 32767

  stream.writeInt16(expected)

  t.is(stream.readInt16(), expected)
})

test('Uint16', (t) => {
  const expected = 65535

  stream.writeUint16(expected)

  t.is(stream.readUint16(), expected)
})

test('Int32', (t) => {
  const expected = 2147483647

  stream.writeInt32(expected)

  t.is(stream.readInt32(), expected)
})

test('Uint32', (t) => {
  const expected = 4294967295

  stream.writeUint32(expected)

  t.is(stream.readUint32(), expected)
})

test('Int64', (t) => {
  const expected = 9223372036854775807n

  stream.writeInt64(expected)

  t.is(stream.readInt64(), expected)
})

test('Uint64', (t) => {
  const expected = 18446744073709551615n

  stream.writeUint64(expected)

  t.is(stream.readUint64(), expected)
})

test('ZigZag', (t) => {
  const expected = 1234567890

  stream.writeZigZag(expected)

  t.is(stream.readZigZag(), expected)
})

test('ZigZong', (t) => {
  const expected = 1234567890n
  stream.writeZigZong(expected)

  t.is(stream.readZigZong(), expected)
})

test('VarInt', (t) => {
  const expected = 1234567890

  stream.writeVarInt(expected)

  t.is(stream.readVarInt(), expected)
})

test('VarLong', (t) => {
  const expected = 1234567890n

  stream.writeVarLong(expected)

  t.is(stream.readVarLong(), expected)
})

test('Float32', (t) => {
  const expected = 1234

  stream.writeFloat32(expected)

  t.is(stream.readFloat32(), expected)
})

test('Float64', (t) => {
  const expected = 123.456

  stream.writeFloat64(expected)

  t.is(stream.readFloat64(), expected)
})

test('String16', (t) => {
  const expected = "Hello, World!"

  stream.writeString16(expected)

  t.is(stream.readString16(), expected)
})

test('String32', (t) => {
  const expected = "Hello, World!"

  stream.writeString32(expected)

  t.is(stream.readString32(), expected)
})

test('VarString', (t) => {
  const expected = "Hello, World!"

  stream.writeVarString(expected)

  t.is(stream.readVarString(), expected)
})