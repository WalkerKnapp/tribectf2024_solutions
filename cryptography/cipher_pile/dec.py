
import gzip
import os

if not os.path.exists("outputs"):
    os.mkdir("outputs")

plain = open("plain", 'rb').read()

# Read in encoded input files
encs = []

for name in range(ord('a'), ord('h') + 1):
    name = chr(name)

    encs.append(open(name + ".enc", 'rb').read())

# Search for the encoded file corresponding with the given plain file
for i in range(len(encs)):
    print("Trying pair", i)

    # Assuming that this pair matches, the encryption pad can be found by XORing the two
    key_attempt = [encs[i][j] ^ plain[j] for j in range(len(encs[i]))]

    # If this pad is correct, applying it to all other encoded files will yield valid gzip streams
    invalid = False
    for j in range(len(encs)):
        decoded = [encs[j][k] ^ key_attempt[k] for k in range(len(key_attempt))]

        try:
            gzipped = gzip.decompress(bytes(decoded))

            f = open(f"outputs/{j}.txt", 'wb')
            f.write(gzipped)
            f.close()
        except:
            print("Pair", i, "is invalid.")
            invalid = True
            break

    # If we found the correct key, stop trying others
    if not invalid:
        print("Found key! Outputs are written to 'outputs'")
        break

# Extract the components of the flag from each output
print("Flag: ", end='')

for i in range(len(encs)):
    with open(f"outputs/{i}.txt") as f:
        # The flag is located between asterisks, the only such string in all files
        chunks = [x for x in f.read().split("*") if len(x) > 0]
        print(chunks[1].strip(), end='')

print()
