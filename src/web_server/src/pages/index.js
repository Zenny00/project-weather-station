function get_cities() {
  fetch("http://localhost:8080/get_cities")
    .then(response => response.json())
    .then(data => console.log(data))
    .catch(error => console.log(error));
}

async function add_cities_to_select(selectId) {
  const cities = await fetch("http://localhost:8080/get_cities_from_db")
    .then(response => response.json())
    .then(data => { return data })
    .catch(error => console.log(error));

  var option = document.createElement("option");
  option.text = cities[0].city + ", " + cities[0].state;
  option.value = cities[0].city; 
  var select = document.getElementById(selectId);
  select.appendChild(option);
}
