Values are little-endian. The raw file blocks are always padded to the nearest multiple of 2048. Not sure if it's really required though.

```
68 bytes HEADER
  str[8]      magic (4d 50 4b 00 00 00 02 00)
  uint(4/8?)  file_count
  56/52?      padding

file_count * 256 bytes FILE_HEADER
  uint4     file_index
  uint8     begin_address
  uint8     length
  uint8     length
  str[228]  file_path

RAW_DATA
  ```
