import leaflet, { Map, type LatLngExpression } from "leaflet";

let map;

const initialCoordinates = [60.2, 25.0];

export function initOsmMap(element: HTMLElement, initialCoordinates: LatLngExpression) {
    console.log(element)
    let map = leaflet.map(element, {
        renderer: leaflet.canvas()
    });
    map.setView(initialCoordinates, 13);

    leaflet.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    return map
}

export function addStopMarker(stop: { stop_lat: number; stop_lon: number; }, map: Map) {
    leaflet.circleMarker([stop.stop_lat, stop.stop_lon]).addTo(map)
}
