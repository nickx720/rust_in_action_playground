# Tar Header Notes (Quick Review)

## Big Picture
- A tar file is just bytes on disk.
- It is organized into 512-byte blocks.
- Each file entry is:
  - 512-byte header
  - file data of length `size`
  - padding with NULs so data ends on a 512-byte boundary

## Header Fields Are Bytes
- The header is 512 bytes, split into fixed fields by offsets.
- Each field is just a byte slice. The meaning depends on the field.

### String Fields
- Example: `name[100]`
- Bytes are treated as text (ASCII/UTF-8) and NUL-terminated.
- Read until the first NUL (`0`) byte.

### Numeric Fields
- Example: `size[12]`, `uid[8]`, `gid[8]`, `mtime[12]`
- Stored as ASCII digits representing an **octal** number.
- Valid digits are `0`..`7` only.
- Field is fixed width and padded with NUL (`0`) or space (`' '`).

## Why Octal?
- Historical/compatibility choice in the tar format.
- ASCII text avoids endianness/word-size issues.
- Once standardized, tools must follow it.

## Example: Parsing `size`
Bytes in the header field:

```
30 30 30 31 30 31 00 00 00 00 00 00
```

Characters:

```
'0''0''0''1''0''1'\0\0\0\0\0\0
```

Steps:
1. Trim trailing NUL/space -> "000101"
2. Parse base-8 -> decimal 65
3. Use 65 as the number of data bytes following the header

## Rust Pattern (Trim Bytes, Then Parse)

```rust
let field = &header[124..136]; // size field
let end = field.iter()
    .rposition(|&b| b != 0 && b != b' ')
    .map(|i| i + 1)
    .unwrap_or(0);
let s = std::str::from_utf8(&field[..end])?;
let size = u64::from_str_radix(s, 8)?;
```

## Seeing NUL Bytes When Debugging
NUL is not printable, so a normal string print won’t show it.
Use debug/hex output:

```rust
println!("{:?}", &header.size);   // shows 0 as 0
println!("{:02x?}", &header.size); // hex bytes, includes 00
```

Or show escapes:

```rust
let s = String::from_utf8_lossy(&header.size);
println!("{:?}", s); // shows "\u{0}" for NUL
```

## 512-Byte Boundary Calculation
After reading `size` bytes of data, skip padding to the next 512-byte boundary:

```rust
let padded = ((size + 511) / 512) * 512;
// next header starts at current_header + 512 + padded
```
