import test from 'ava'

import { BinaryStream } from '../index.js'

test('sum from native', (t) => {
  const stream = new BinaryStream()

  stream.writeVarString('Hello, World!')

  const word = stream.readVarString()

  t.is(word, 'Hello, World!')
})
