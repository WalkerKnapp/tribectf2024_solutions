with open("griffin.treasure", "rb") as f:
    with open("griffin.treasure.decrypted", "wb") as f2:
        f2.write(bytes([b ^ 0x1a for b in f.read()]))
