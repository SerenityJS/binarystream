# binarystream

Binarystream is a simple package designed to simplify streaming of binary data in Javascript, while being written in Rust.

## Usage
```ts
import { BinaryStream } from '@serenityjs/binarystream'

const stream = new BinaryStream()
stream.writeUint8(255)
stream.writeString('Hello World!')

stream.readUint8() // 255
stream.readString() // Hello World!
```

## Documentation

Documentation can be found [here](https://docs.rs/binarystream)
