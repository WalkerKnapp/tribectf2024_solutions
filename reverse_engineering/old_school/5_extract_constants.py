with open("Reaper.bin", "rb") as f:
    reaper_bin = list(f.read())

    print("Memory from $DE00 -> $DEOF XORed with the value at address $03:")
    xor_constant = reaper_bin[0x03]
    print(bytes([x ^ xor_constant for x in reaper_bin[0xDE00:0xDE0F]]))

    print("Memory from $DD50 -> $DD6e XORed with indirect values from addresses stored at $03 -> $21:")
    payload = []
    for i in range(0x1e):
        # Load A from memory
        A = reaper_bin[0xDD50 + i]

        # Alpha indirect addressing implementation:
        # uint64_t rax_559 = (arg1[1] + r11[1]);
        # bool rdx_379 = rcx[(rcx[rax_559] | (rcx[(rax_559 + 1)] << 8))];

        indirect_address = 0x03 + i
        address = reaper_bin[indirect_address] | (reaper_bin[(indirect_address + 1)] << 8)
        value = reaper_bin[address]

        A ^= value

        payload.append(A)
    print(bytes(payload))

    print("Memory from $DD80 XORed with the value at address $DE:")
    xor_constant = reaper_bin[0xDE]

    payload = []
    i = 0
    while (c := reaper_bin[0xDD80 + i]) != 0x00:
        payload.append(c ^ xor_constant)
        i += 1

    print(bytes(payload))



