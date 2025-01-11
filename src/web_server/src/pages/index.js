function get_cities() {
  fetch("http://localhost:8080/get_cities")
    .then(response => response.json())
    .then(data => console.log(data))
    .catch(error => console.log(error));
}
