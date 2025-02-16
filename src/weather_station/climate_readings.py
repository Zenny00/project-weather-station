from machine import Pin, RTC, I2C, SoftI2C
from utime import sleep, localtime
from bmp085 import BMP180
import dht
# import temperature
import networking
import database
from bh1750 import BH1750
import ntptime
from urandom import getrandbits
import micropg_lite

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
cursor = connection.cursor()

station_id = None
try:
    station_details = open("station_details", "r")
    station_id = [line.strip() for line in station_details.readlines()][0]
except OSError as error:
    print(f"Could not read station details file {error}")
print(station_id)

# Initialize the Real Time Clock (RTC) from the Network Time Protocol (NTP)
ntptime.settime()

# We want to continually log temperature data, we create an array of objects to capture readings
count = 0
while (count < 10):
    sleep(1)
    # Turn on the LED to show a reading is being taken
    board_led.toggle()
    
    # Get the current UTC time
    # Reformat time into string YYYY-MM-DDTHH:mm:ss.SSSZ
    timestamp = localtime()
    timestamp_formatted = f"{timestamp[0]}-{timestamp[1]}-{timestamp[2]}T{timestamp[3]}:{timestamp[4]}:{timestamp[5]}"
    
    # Read temperature, pressure, and altitude from the BMP180 sensor
    temperature, pressure, altitude = round(bmp.temperature, 3), round(bmp.pressure / 100, 4), round(bmp.altitude, 3)
    lux = round(light_sensor.luminance(BH1750.CONT_HIRES_1), 4)
    
    # Measure humidity from the DHT11
    sensor.measure()
    humidity = round(sensor.humidity(), 3)
    print(temperature, pressure, altitude, lux)
    
    # Can only generate a 32-bit number on the Pi Pico so we combine two to make a 64-bit
    high_bits = getrandbits(32)
    low_bits = getrandbits(32)
    
    id = (high_bits << 32) | low_bits # Shift bits left and bitwise OR
    
    try:
        cursor.execute('INSERT INTO measurement (station_id, measurement_id, temperature, pressure, humidity, light_level, timestamp) VALUES (%s, %s, %s, %s, %s, %s, %s)', [str(station_id), str(id), str(temperature), str(pressure), str(humidity), str(lux), str(timestamp_formatted)])
        connection.commit()
    except Exception as e:
        print(f"Error inserting data: {e}")
        
    count += 1
    sleep(0.5)
    board_led.toggle()
    


connection.close()
