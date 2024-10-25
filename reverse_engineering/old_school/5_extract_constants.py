with open("Reaper.bin", "rb") as f:
    print("Memory from $DE00 -> $DEOF XORed with 0x03:")
    f.seek(0xDE00)
    print(bytes([x ^ 0x03 for x in f.read(0x0F)]))
