#! /usr/bin/env python3
# xxd -e -l 32 -g 2 -c 2 -s 0x10000

import struct
import sys

with open(sys.argv[1], 'r+b') as f:
    hex_file = f.read()
    sp = struct.unpack('<I', hex_file[0x10000 : 0x10004])[0]
    checksum = 0
    for i in range(0x10000, 0x1001c, 0x4):
        checksum = checksum + struct.unpack('<I', hex_file[i : i + 0x4])[0]
    f.seek(0x1001c)
    print(f'0x{(~checksum + 1) & 0xFFFFFFFF:x}')
    f.write(struct.pack('<I', (~checksum + 1) & 0xFFFFFFFF))
