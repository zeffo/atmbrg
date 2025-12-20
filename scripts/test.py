import json
import socket
import time

UDP_IP = "0.0.0.0"
UDP_PORT = 5625
COMMAND_PORT = 5600
BUFFER_SIZE = 4096

state = True


device = None
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind((UDP_IP, UDP_PORT))


while True:
    data, addr = sock.recvfrom(BUFFER_SIZE)
    ip, port = addr
    content = data.decode(errors='replace')
    if len(content) <= 15:
        mac = content[:12]
        device = (mac, ip)

    if device:
        message = json.dumps({
            "led": state
        }).encode("utf-8")
        sock.sendto(message, (device[1], COMMAND_PORT))
        state = not state
        time.sleep(2)

