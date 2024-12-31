def measure_temperature_humidity(timestamp, sensor):
    try:
        # Reading is in celsius, convert to Fahrenheit
        temperature_reading = (sensor.temperature() * (9/5) + 32)
        humidity_reading = sensor.humidity()
        reading = {
            "temperature": temperature_reading,
            "humidity": humidity_reading,
            "timestamp": timestamp,
        }
    except OSError as error:
        print(f'Failed to read the from sensor: {error}')


    return reading