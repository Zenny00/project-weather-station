import network
from time import sleep

def connect_to_network(ssid, password):
    # Connect to the WiFi network
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    wlan.connect(ssid, password)
    
    # Wait for Wi-Fi connection
    connection_timeout = 20
    while connection_timeout > 0:
        if wlan.status() >= 3:
            break
        connection_timeout -= 1
        print("Waiting for Wi-Fi connection...")
        sleep(1)

    # Check if connection is successful
    if wlan.status() != 3:
        print("Failed to establish a network connection")
    else:
        print("Connection successful!")