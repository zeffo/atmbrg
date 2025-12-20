import json
import socket
import time

UDP_IP = "0.0.0.0"
UDP_PORT = 5625
COMMAND_PORT = 5600
BUFFER_SIZE = 4096
DEVICE_IP = "192.168.1.160"

state = True


# sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
# sock.bind((UDP_IP, UDP_PORT))
#
#
# while True:
#     data, addr = sock.recvfrom(BUFFER_SIZE)
#     ip, port = addr
#     content = data.decode(errors='replace')
#     if len(content) <= 15:
#         mac = content[:12]
#         device = (mac, ip)
#         print("found device: ", ip, mac)
#
#     if device:
#         message = json.dumps({
#             "led": state
#         }).encode("utf-8")
#         sock.sendto(message, (device[1], COMMAND_PORT))
#         state = not state
#         time.sleep(2)

def send_command(): 
    # JSON command to send
    command = {
        "led": True
    }

    # Convert to JSON string and encode to bytes
    message = json.dumps(command).encode('utf-8')

    # Create UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Send the message
    sock.sendto(message, (DEVICE_IP, COMMAND_PORT))
    print(f"Sent JSON command to {DEVICE_IP}:{COMMAND_PORT}")

    # Close the socket
    sock.close()

send_command()
