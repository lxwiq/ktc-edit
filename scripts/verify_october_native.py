"""Emulate the actual October ARM64 trampolines; requires Python unicorn.

Input is a COPY emitted by KTC_TEST_PATCH_OUTPUT during cargo test, never the
installed game. This does not replace an in-game test of Unity/Steam behavior.
"""
import struct
import sys
from pathlib import Path

from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm64_const import *

data = Path(sys.argv[1]).read_bytes()
off = next(struct.unpack_from('>I', data, 16 + i * 20)[0]
           for i in range(struct.unpack_from('>I', data, 4)[0])
           if struct.unpack_from('>I', data, 8 + i * 20)[0] == 0x100000c)
coin_cave, camp_cave = 0x44cdf00, 0x44cdf40
coin_return, camp_restart, fade = 0xb15cd8, 0x660e78, 0x661ef8
callback = 0x701748


def machine():
    u = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
    for page in {coin_cave & ~0xfff, (camp_cave + 243) & ~0xfff, coin_return & ~0xfff,
                 camp_restart & ~0xfff, fade & ~0xfff}:
        u.mem_map(page, 4096)
    u.mem_write(coin_cave, data[off + coin_cave:off + coin_cave + 24])
    u.mem_write(camp_cave, data[off + camp_cave:off + camp_cave + 244])
    u.mem_map(0x1000000, 0x10000)
    u.mem_map(0x2000000, 0x10000)
    u.reg_write(UC_ARM64_REG_SP, 0x2008000)
    return u


def run(u, start, stops):
    reached = []
    def stop(emu, address, _size, _context):
        if address in stops:
            reached.append(address)
            emu.emu_stop()
    u.hook_add(UC_HOOK_CODE, stop)
    u.emu_start(start, 0, count=1000)
    assert len(reached) == 1, 'Trampoline did not reach an expected exit'
    return reached[0]


def put(u, address, value, fmt='<Q'):
    u.mem_write(address, struct.pack(fmt, value))


def get(u, address, fmt='<Q'):
    return struct.unpack(fmt, u.mem_read(address, struct.calcsize(fmt)))[0]


# Packed currency enum in low 32 bits, logical amount in high 32 bits.
for currency in [0, 1, 2]:
    for amount in [0, 1, 29, 30, 31, 99, 3000, 1_000_000]:
        u = machine()
        u.reg_write(UC_ARM64_REG_X21, (amount << 32) | currency)
        u.reg_write(UC_ARM64_REG_X8, 17)
        assert run(u, coin_cave, {coin_return}) == coin_return
        expected = min(amount, 30) if currency == 0 else amount
        assert u.reg_read(UC_ARM64_REG_W26) == expected
        assert u.reg_read(UC_ARM64_REG_X8) == 17
        assert u.reg_read(UC_ARM64_REG_X21) == (amount << 32) | currency
print('24 coin/gem cases passed: target capped, logical balance untouched')

# Same circular-list layout as the native LinkedList.Remove inspected in this build.
forest, linked, item, event = 0x1000000, 0x1001000, 0x1002000, 0x1003000
nodes = [0x1004000, 0x1004100, 0x1004200]
for count, removed in [(1, 0), (2, 0), (2, 1), (3, 0), (3, 1), (3, 2)]:
    u = machine()
    put(u, forest + 0x28, linked)
    put(u, linked + 0x10, nodes[0])
    put(u, linked + 0x18, count, '<I')
    put(u, linked + 0x1c, 7, '<I')
    for i in range(count):
        put(u, nodes[i] + 0x10, linked)
        put(u, nodes[i] + 0x18, nodes[(i + 1) % count])
        put(u, nodes[i] + 0x20, nodes[(i - 1) % count])
    put(u, item + 0x40, event)
    put(u, event + 0x10, callback)
    put(u, item + 0x60, nodes[removed])
    put(u, item + 0x38, forest)
    u.reg_write(UC_ARM64_REG_X0, item)
    u.reg_write(UC_ARM64_REG_X21, item)
    u.reg_write(UC_ARM64_REG_X19, forest)
    assert run(u, camp_cave, {camp_restart, fade}) == camp_restart
    assert u.reg_read(UC_ARM64_REG_X0) == forest
    assert u.reg_read(UC_ARM64_REG_SP) == 0x20080d0
    assert get(u, linked + 0x18, '<I') == count - 1
    assert get(u, linked + 0x1c, '<I') == 8
    remaining = [n for i, n in enumerate(nodes[:count]) if i != removed]
    assert get(u, linked + 0x10) == (remaining[0] if remaining else 0)
    for i, node in enumerate(remaining):
        assert get(u, node + 0x18) == remaining[(i + 1) % len(remaining)]
        assert get(u, node + 0x20) == remaining[(i - 1) % len(remaining)]
    for relative in [0x10, 0x18, 0x20]:
        assert get(u, nodes[removed] + relative) == 0
    assert get(u, item + 0x60) == get(u, item + 0x38) == 0
    assert get(u, item + 0x40) == event  # camp/event remains alive
print('6 camp head/middle/tail/single-node unlink cases passed')

for kind in ['no_event', 'other_callback', 'multicast_match', 'no_node']:
    u = machine()
    if kind != 'no_event':
        put(u, item + 0x40, event)
        put(u, event + 0x10, callback if kind == 'no_node' else 0x1234)
    if kind == 'multicast_match':
        array, delegate = 0x1006000, 0x1007000
        put(u, event + 0x78, array)
        put(u, array + 0x18, 1, '<I')
        put(u, array + 0x20, delegate)
        put(u, delegate + 0x10, callback)
    u.reg_write(UC_ARM64_REG_X0, item)
    u.reg_write(UC_ARM64_REG_X21, item)
    u.reg_write(UC_ARM64_REG_X19, forest)
    expected = camp_restart if kind in ['multicast_match', 'no_node'] else fade
    assert run(u, camp_cave, {camp_restart, fade}) == expected
print('4 callback/fallback/no-node cases passed')
