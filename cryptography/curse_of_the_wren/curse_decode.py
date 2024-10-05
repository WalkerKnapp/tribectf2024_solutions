import base64

import enigma.plugboard
from enigma.rotor import Rotors

# Initial data
data0 = open("M3UKWCVI16P8XIIIXXX12LVIII6F13MBQCRDIEJKWMTOSPXUZGH").read()
print(data0)

# data0 is encoded as hex bytes
data1 = [int(x, 16) for x in data0.split()]
print(bytes(data1))

# data1 is encoded as base64 (minus header)
data2 = base64.b64decode(bytes(data1[7:]))
print(data2)

# data2 is encoded as octal bytes (mins header)
data3 = list(map(lambda x: int(x, 8), data2[7:].split()))
print(bytes(data3))

# data3 is XOR'd with a repeating pad of 0x1693
data4 = data3[27:]
data4 = [data4[i] ^ (0x16 if i % 2 == 0 else 0x93) for i in range(len(data4))]
print(bytes(data4))

# data4 is passed through an enimga cipher with parameters dictated in the filename:
# M3
# UKW C
# VI - 16P - 8X
# III - XXX - 12L
# VIII - 6F - 13M
# BQ CR DI EJ KW MT OS PX UZ GH
data5 = bytes(data4[27:]).decode('ascii')

machine = enigma.machine.Machine(
    "M3",
    [Rotors.M3.ETW, Rotors.M3.VIII, Rotors.M3.III, Rotors.M3.VI, Rotors.M3.UKWC],
    enigma.plugboard.Plugboard("BQ CR DI EJ KW MT OS PX UZ GH")
)
machine.set_rotor_position("VI", "P")
machine.set_rotor_ringstellung("VI", "H")
machine.set_rotor_position("III", "X")
machine.set_rotor_ringstellung("III", "L")
machine.set_rotor_position("VIII", "F")
machine.set_rotor_ringstellung("VIII", "M")

data5 = machine.encode(data5, split=0)

# Data5 describes the format for the key in plain language.
print(data5)

