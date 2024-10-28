message = []

with open("problem/e2f4c680.bmp", 'rb') as file:
    # Skip the header
    file.seek(54)

    while True:
        # Read the `offset` value (number of bytes of noise printed after the byte from the message)
        offset = int.from_bytes(file.read(1), 'little')

        if offset == 255:
            break

        # Read the byte from the message
        val = int.from_bytes(file.read(1), 'little')
        message.append(val)

        # Skip the `offset - 1` bytes of noise
        file.read(offset - 1)

# Print the message as UTF-8
print(bytes(message).decode('utf-8'))

