import pyshark

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
