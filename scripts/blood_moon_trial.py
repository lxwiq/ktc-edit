"""Experimental October ARM64 patch; writes a NEW dylib, never the source.

Forces LandData.GetDayTypeForDay to return DayType.bossDay (4).
This is a static trial, not proof of nightly waves or challenge compatibility.
No code cave: guarded day, wave-selector and both-sides instruction replacements.
"""
import argparse
from pathlib import Path
import struct

RVA = 0x9A8554
EXPECTED = bytes.fromhex('ff0301d1 f65701a9')
PATCH = bytes.fromhex('80008052 c0035fd6')


def patch(data):
    out = bytearray(data)
    if data[:4] != bytes.fromhex('cafebabe'):
        raise ValueError('Expected the verified universal Mach-O format')
    count = struct.unpack_from('>I', data, 4)[0]
    slices = [struct.unpack_from('>5I', data, 8 + i * 20) for i in range(count)]
    arm = [s for s in slices if s[0] == 0x100000C]
    if len(arm) != 1:
        raise ValueError('Expected exactly one ARM64 slice')
    _, _, base, size, _ = arm[0]
    # Resolve RVA through the Mach-O segment table, not an assumed VA=file offset.
    if struct.unpack_from('<I', data, base)[0] != 0xFEEDFACF:
        raise ValueError('Expected 64-bit little-endian Mach-O')
    cursor = base + 32
    location = None
    for _ in range(struct.unpack_from('<I', data, base + 16)[0]):
        cmd, length = struct.unpack_from('<II', data, cursor)
        if cmd == 0x19:
            vmaddr, vmsize, fileoff, filesize = struct.unpack_from('<4Q', data, cursor + 24)
            if vmaddr <= RVA and RVA + 8 <= vmaddr + filesize:
                location = base + fileoff + RVA - vmaddr
                initprot = struct.unpack_from('<I', data, cursor + 60)[0]
                if not initprot & 4:
                    raise ValueError('Target segment is not executable')
        cursor += length
    if location is None or location + 8 > base + size:
        raise ValueError('Method not mapped into ARM64 file slice')
    if data[location:location+8] != EXPECTED:
        raise ValueError('Unsupported method bytes; refusing to patch')
    out[location:location+8] = PATCH
    sites = [(0x61220c,0xf9401502),(0x61226c,0xf9400d02),(0x6122b0,0xf9400d02)]
    allowed = set(range(location,location+8))
    for rva, expected in sites:
        at = base+rva
        assert struct.unpack_from('<I',data,at)[0] == expected
        assert struct.unpack_from('<I',data,at+4)[0] == 0xaa1403e0
        struct.pack_into('<I',out,at,0xf9401102)
        allowed.update(range(at,at+4))
    at = base + 0x613064
    if struct.unpack_from('<I', data, at)[0] != 0x3941b03a or struct.unpack_from('<I', data, at+4)[0] != 0xd0022b99:
        raise ValueError('Unsupported ScheduleWave side selector')
    struct.pack_into('<I',out,at,0x5280003a)
    allowed.update(range(at,at+4))
    assert len(out) == len(data)
    assert all(a==b or i in allowed for i,(a,b) in enumerate(zip(data,out)))
    return bytes(out), location


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    if args.source.resolve() == args.output.resolve():
        parser.error('Output must be a separate copy')
    result, offset = patch(args.source.read_bytes())
    with args.output.open('xb') as f:
        f.write(result)
    print(f'Created {args.output}; patched 24 bytes across five guarded sites; day method at {offset:#x}')
    print('Not installed or signed. Existing saved schedules need the editor conversion.')
