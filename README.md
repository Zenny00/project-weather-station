# Project Weather Station

A self hosted weather monitoring system reporting local weather data viewable via a typical web browser.

## About

### Project Purpose

The goal of this project is to gain more familiarity with embedded programming and associated development environments, hardware and serial communication, database hosting and management, and end-to-end infrastucture systems.


The project will utilize several readily available embedded hardware devices such as the Raspberry Pi Pico and Arduino Uno. Throughout this project I will be using several different programming languages in order to become more familiar with them and how they are used within the embedded sphere.


The minimum standard of completeness for this project will be to build a weather station that is able to withstand the outdoor environment with a self sufficient power source. This device will take measurements and send them to a separate device which will be hosting a database and web-server accessable on at least the local network.


Nice to haves would include:
- An interactive website that allows for exploration of the data and visualizations.
- Allowing the website to be accessable outside of the local network.
- Providing access to historical data.

### Equipment:

This project will require several hardware components that I do not currently own and thus will need to purchase. The list of said hardware resources will be tracked using the list below:


- [ ] [Arduino Uno](https://www.amazon.com/Arduino-A000066-ARDUINO-UNO-R3/dp/B008GRTSV6)
- [ ] [Anemometer](https://moderndevice.com/products/wind-sensor?variant=42521581158642&country=US&currency=USD&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&gQT=2)
- [ ] [Pressure Sensor](https://www.adafruit.com/product/2651?gQT=2)
- [ ] [Temperature Sensor](https://www.adafruit.com/product/1899?gQT=2)
- [x] [Raspberry Pi Pico W](https://www.amazon.com/Raspberry-Pi-Pico-Wireless-Bluetooth/dp/B0B5H17CMK/ref=sr_1_3?crid=3L2MKP6HIN3ZH&dib=eyJ2IjoiMSJ9.TRC16-4mmKR3cHUH3tLkYqwTaizkohLAInTj4zyLoJQrMK7AJHOTgfdzlYV5QXjf6eek4zJRsfYOfbehadvxvtQdCHPriARZ8wKxUDXOenSYlwz0gysoejFjHHfwZNYlCWHJq9rekmhFESqr7eLaXacTbQJCTHyUYK1pixmPk0_eKMEF9TsY7WuE2nWdPQHWYlU2AQ9hnJWJqGf_7608wDFccfBADF2W41zwlgp7r2jAofCloOZB11SobEBxv17ZjoISaSgkFWXRJ5YvzUJTtQY1FVDdEBTtDxYG_7kN-0Y.YKPqaJMVDK9X_gOIkAoMu8inxNZYMleIKsgYjti2Jg8&dib_tag=se&keywords=raspberry+pi+pico&qid=1734203484&s=electronics&sprefix=raspberry+pi+pico%2Celectronics%2C115&sr=1-3)

### Project Diagram

![An overview of the project architecture](./docs/images/project-weather-station-diagram.png)
