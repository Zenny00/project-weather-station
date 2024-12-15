from machine import Pin
from utime import sleep, localtime
import dht

# The object representing the DHT11 temperature and humidity sensor
sensor = dht.DHT11(Pin(22))
# The LED on the board, used to indicate when a reading is being taken
board_led = Pin("LED", Pin.OUT)

# We want to continually log temperature data, we create an array of objects to capture readings
readings = []
while (len(readings) < 10):
    try:
        sleep(9.5)
        # Turn on the LED to show a reading is being taken
        board_led.toggle()
        sensor.measure()
        # Reading is in celsius, convert to Fahrenheit
        timestamp = localtime()
        temperature_reading = (sensor.temperature() * (9/5) + 32)
        humidity_reading = sensor.humidity()
        readings.append({
            "temperature": temperature_reading,
            "humidity": humidity_reading,
            "timestamp": timestamp,
        })
        sleep(0.5)
        board_led.toggle()
        
    except OSError as error:
        print(f'Failed to read the from sensor: {error}')

print(readings)
