// Overall options for the application
const applicationState = {
  temperature_unit: "F",
  pressure_unit: "mB",
  humidity_unit: "%",
  precipitation_unit: "in.",
  wind_speed_unit: "m/s",
  get_state() {
    return { ...this };
  }
};

async function add_cities_to_select(selectId) {
  // Fetch the data from the backend API
  const cities = await fetch("http://localhost:8080/get_cities_from_db")
    .then(response => response.json())
    .then(data => { return data })
    .catch(error => console.log(error));
  
  // Get the selector element from the page
  var select = document.getElementById(selectId);
  
  // Add an option to the dropdown for each location
  for (var i = 0; i < cities.length; i++) {
    var option = document.createElement("option");
    option.text = cities[i].city + ", " + cities[i].state;
    option.value = cities[i].location_id; 
    select.appendChild(option);
  }

  // Trigger the initial on change event for the 
  select.dispatchEvent(new Event("change", { "bubbles": true }));
}

async function get_stations_from_location(location_id) {
  // Fetch the data from the backend API
  const stations = await fetch("http://localhost:8080/get_stations_from_db?location_id=" + location_id)
    .then(response => response.json())
    .then(data => { return data })
    .catch(error => console.log(error));

  return stations[0];
}

async function get_station_measurements(station_id) {
  // Get the latest measurements from the station
  const measurements = await fetch("http://localhost:8080/get_measurements_from_station?station_id=" + station_id)
    .then(response => response.json())
    .then(data => { return data })
    .catch(error => console.log(error));

  return measurements;
}

async function display_measurements_on_dashboard(select) {
  // Get the location from the user selection
  const location_id = select.value;

  // Get the station associated with the user selected locations
  const station = await get_stations_from_location(location_id);

  // If no station is found no data should be displayed
  if (!station) {
    return;
  }

  // Get the latest (6) measurements from this station to be displayed on the dashboard
  const measurements = await get_station_measurements(station.station_id);
  
  // Display the measurement data in each box
  const componentIdPrefix = "current_conditions";
  for (let i = 0; i < measurements.length; i++) {
    let measurement = measurements[i];
    let componentId = componentIdPrefix + "_" + i;
    display_title(measurement, componentId);
    display_conditions(measurement, componentId);
  }
}

function display_title(measurement, componentId) {
  // Get the measurement timestamp and format it for display
  const timestamp = new Date(measurement.timestamp).toLocaleString('en-US', { day: "numeric", month: "short" , hour: "numeric" });
  let header = document.getElementById(componentId + "_header");
  header.innerText = timestamp;
}

function display_conditions(measurement, componentId) {
  const state = applicationState.get_state();

  // Get values to be displayed from the measurement and format for display

  // Get the unit to determine how to display temperature
  let temperature;
  if (state.temperature_unit === "F") {
    temperature = (measurement.temperature * (9/5)) + 32;
  } else {
    temperature = measurement.temperature;
  }
  temperature = temperature.toFixed(1) + " °" + state.temperature_unit;

  // Get the unit to determine how to display humidity
  let humidity;
  if (state.humidity_unit === "%") {
    humidity = measurement.humidity;
  }
  humidity = humidity.toFixed(1) + state.humidity_unit;
  
  // Get the unit to determine how to display wind speed
  let wind_speed;
  if (state.wind_speed_unit === "m/s") {
    wind_speed = measurement.wind_speed;
  }
  wind_speed = wind_speed.toFixed(1) + state.wind_speed_unit;
  
  // Get the unit to determine how to display precipitation
  let precipitation;
  if (state.precipitation_unit === "in.") {
    precipitation = measurement.precipitation / 2.54;
  } else {
    precipitation = measurement.precipitation;
  }
  precipitation = precipitation.toFixed(1) + state.precipitation_unit;
  

  let conditions_text = document.getElementById(componentId + "_text");
  conditions_text.innerText = temperature;

}
