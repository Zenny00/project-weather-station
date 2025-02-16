from machine import Pin, RTC, I2C, SoftI2C
from utime import sleep, localtime
from bmp085 import BMP180
import dht
import temperature
import networking
import database
from bh1750 import BH1750
import ntptime

# The object representing the DHT11 temperature and humidity sensor
sensor = dht.DHT11(Pin(22))

# The LED on the board, used to indicate when a reading is being taken
board_led = Pin("LED", Pin.OUT)

# The I2C interface over PIN 0 and 1 for the BMP180
i2c_bmp = I2C(0, sda = Pin(0), scl = Pin(1), freq = 1000)

# The I2C interface over PIN 5 and 4 for the BH1750
i2c_lux = SoftI2C(scl=Pin(5), sda=Pin(4), freq=400000)

# The object representing the BMP180 sensor
bmp = None
try:
    bmp = BMP180(i2c_bmp)
    bmp.oversample = 2
    bmp.sealevel = 101325 # Standard sea level pressure in Pascals (1013.25 mB)
except:
   print("Can't read BMP180")
light_sensor = BH1750(bus=i2c_lux, addr=0x23)

# Connect to the wifi network
ssid = None
password = None
try:
    wifi_credentials = open("wifi_credentials", "r")
    ssid, password = [line.strip() for line in wifi_credentials.readlines()]
    networking.connect_to_network(ssid, password)
except OSError as error:
    print(f"Could not read WiFi credential file {error}")

# Establish a connection with the database
connection = database.open_db_connection("./db_credentials")
cur = connection.cursor()

# Initialize the Real Time Clock (RTC) from the Network Time Protocol (NTP)
ntptime.settime()

# We want to continually log temperature data, we create an array of objects to capture readings
readings = []
while (len(readings) < 10):
    sleep(1)
    # Turn on the LED to show a reading is being taken
    board_led.toggle()
    
    # Get the current UTC time
    # Reformat time into string YYYY-MM-DDTHH:mm:ss.SSSZ
    timestamp = localtime()
    timestamp_formatted = f"{timestamp[0]}-{timestamp[1]}-{timestamp[2]}T{timestamp[3]}:{timestamp[4]}:{timestamp[5]}"
    
    # Read temperature, pressure, and altitude from the BMP180 sensor
    temperature, pressure, altitude = bmp.temperature, bmp.pressure, bmp.altitude
    lux = light_sensor.luminance(BH1750.CONT_HIRES_1)
    
    sleep(0.5)
    board_led.toggle()
    
print(readings)

connection.close()
