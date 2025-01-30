function get_cities() {
  fetch("http://localhost:8080/get_cities")
    .then(response => response.json())
    .then(data => console.log(data))
    .catch(error => console.log(error));
}

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
}

async function get_stations_from_location(location_id) {
  // Fetch the data from the backend API
  const stations = await fetch("http://localhost:8080/get_stations_from_location?=" + location_id)
    .then(response => response.json())
    .then(data => { return data })
    .catch(error => console.log(error));

  return stations[0];
}
