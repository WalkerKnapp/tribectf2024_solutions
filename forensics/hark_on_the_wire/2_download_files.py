import os.path

import pyshark

if not os.path.exists("output"):
    os.mkdir("output")

output_file = None
expecting_filename = False

cap = pyshark.FileCapture("./hark_on_the_wire.pcapng")
for packet in cap:
    # Skip non-IP packets and non-TCP packets
    if not hasattr(packet, 'ip'):
        continue
    if not hasattr(packet, 'tcp'):
        continue

    # Skip TCP packets that don't push any data
    if packet.tcp.flags_push == 'False':
        continue

    if packet.ip.src == '127.0.0.1':
        # packet.tcp.payload is formatted as a hex-string separated by ":"
        payload = [int(x, 16) for x in packet.tcp.payload.split(":")]
        print(bytes(payload))

        if output_file is not None:
            if bytes(payload).endswith(b'DONE'):
                output_file.write(bytes(payload).strip(b'DONE'))
                output_file.close()
                output_file = None
            else:
                output_file.write(bytes(payload))

        elif expecting_filename:
            output_file = open("output/" + bytes(payload).decode('ascii'), "wb")
            expecting_filename = False

        else:
            # Each file transfer starts with a valid integer
            try:
                int(bytes(payload).decode('ascii'))
                expecting_filename = True
            except ValueError:
                continue

