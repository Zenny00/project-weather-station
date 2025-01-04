# Database Schema

The database will have three tables initially, Station, Location, and Measurement:

### Station:

The station table will represent the physical weather station.
- **station_id**: A unique identifier for the weather station
- **location_id**: A foreign key mapping to the location table. This is the physical location of the station
- **name**: The name of the weather station
- **start_date**: The date the station was established

### Location:

Represents a physical location where a station has been established.

- **location_id**: A unique identifier for this location
- **latitude**: The latitude of this location
- **longitude**: The longitude of this location
- **elevation**: The elevation above sea level for this location
- **city**: The city from the physical address
- **country**: The country in which this location resides
- **state**: The state code for this location (US based)

### Measurement:

A single measurement taken by a weather station

- **measurement_id**: Uniquely identifies this measurement
- **station_id**: Foreign key used to map this measurement to its parent station
- **timestamp**: The date and time at which this measurement was taken
- **temperature**: The temperature at the station (degrees celsius)
- **humidity**: The relative humidity at the station (percentage)
- **precipitation**: The total precipitation at the station since the last measurement (cm)
- **pressure**: The pressure at the station (mb)
- **wind_speed**: The wind speed at the station (m/s)

