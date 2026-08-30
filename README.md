## GGUF structure

GGUF
├── Header
│ ├── Magic: "GGUF" (4 bytes)
│ ├── Version: u32 (little-endian)
│ ├── Tensor Count: u64
│ └── Metadata Count: u64
├── Metadata
│ ├── Key Length: u64
│ ├── Key: [u8; key_length]
│ ├── Value Type: u32 (0-12)
│ └── Value: depends on type
├── Tensor Information
└── Tensor Data

## Supported Value Types

| ID | Type    | Size    |
|----|---------|---------|
| 0  | UInt8   | 1 byte  |
| 1  | Int8    | 1 byte  |
| 2  | UInt16  | 2 bytes |
| 3  | Int16   | 2 bytes |
| 4  | UInt32  | 4 bytes |
| 5  | Int32   | 4 bytes |
| 6  | Float32 | 4 bytes |
| 7  | Bool    | 1 byte  |
| 8  | String  | 8 byte  |
| 9  | Array   | dynamic |
| 10 | UInt64  | 8 bytes |
| 11 | Int64   | 8 bytes |
| 12 | Float64 | 8 bytes |

An array of 3 UInt32 values:
- Element Type: 4 (UInt32)
- Array Size: 3
- Elements: [100, 200, 300]

## Usage
GGUF_LOC_KEY=/path/to/model.gguf cargo run