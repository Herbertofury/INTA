#!/usr/bin/env python3
"""Render the idle native field as an offline, scalable XFCE session backdrop."""
import ctypes
import sys
from pathlib import Path


def render(library: Path, output: Path):
    field = ctypes.CDLL(str(library.resolve())).inta_optics_cell
    field.argtypes = [ctypes.c_uint32] * 5 + [ctypes.c_uint64] * 2
    field.restype = ctypes.c_uint32
    cols, rows = 104, 48
    with output.open("w", encoding="ascii", newline="\n") as svg:
        svg.write('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1664 960" preserveAspectRatio="xMidYMid slice">\n')
        svg.write('<defs><radialGradient id="sky" cx=".68" cy=".39" r=".75"><stop stop-color="#1b2a3c"/>'
                  '<stop offset=".54" stop-color="#111925"/><stop offset="1" stop-color="#080c12"/></radialGradient></defs>\n'
                  '<rect width="1664" height="960" fill="url(#sky)"/><g font-family="monospace" font-size="10">\n')
        for row in range(rows):
            for col in range(cols):
                value = field(col, row, cols, rows, 0, 0, 0)
                if value:
                    svg.write(f'<text x="{col * 16 + 4}" y="{row * 20 + 14}" fill="#{value & 0xffffff:06x}">'
                              f'{chr(value >> 24)}</text>\n')
        svg.write('</g></svg>\n')


if __name__ == "__main__":
    render(Path(sys.argv[1]), Path(sys.argv[2]))
