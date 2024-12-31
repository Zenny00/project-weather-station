from machine import Pin, RTC
from utime import sleep, localtime
import dht
import temperature
import network

# The object representing the DHT11 temperature and humidity sensor
sensor = dht.DHT11(Pin(22))
# The LED on the board, used to indicate when a reading is being taken
board_led = Pin("LED", Pin.OUT)

# Connect to the wifi network

# Initialize the Real Time Clock (RTC) from the Network Time Protocol (NTP)

# We want to continually log temperature data, we create an array of objects to capture readings
readings = []
while (len(readings) < 10):
    sleep(1)
    # Turn on the LED to show a reading is being taken
    board_led.toggle()
    sensor.measure()
    
    # Reformat time into string YYYY-MM-DDTHH:mm:ss.SSSZ
    timestamp = localtime()
    timestamp_formatted = f"{timestamp[0]}-{timestamp[1]}-{timestamp[2]}T{timestamp[3]}:{timestamp[4]}:{timestamp[5]}"
    
    readings.append(temperature.measure_temperature_humidity(timestamp=timestamp_formatted, sensor=sensor))
    
    sleep(0.5)
    board_led.toggle()
    
print(readings)