import socket
# Target IP address and port
target_ip = '52.15.167.81'  # Replace with the IP address of the target machine
target_port = 45612  # Replace with the desired port number

# Data to be sent
data_to_send = "data sending from local socket to central test server"

# Create a socket
client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Connect to the target IP and port
client_socket.connect((target_ip, target_port))

# Send data
client_socket.send(data_to_send.encode())

# Close the socket
client_socket.close()

