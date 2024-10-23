with open("griffin.treasure.decrypted", "rb") as f:
    with open("griffin.treasure.trimmed", "wb") as f2:
        f2.write(f.read()[11:])
