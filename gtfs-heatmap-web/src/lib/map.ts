import leaflet, { Map, type LatLngExpression, CircleMarker } from "leaflet";

let map;

export function initOsmMap(element: HTMLElement, initialCoordinates: LatLngExpression) {
    console.log(element)
    let map = leaflet.map(element, {
        renderer: leaflet.canvas()
    });
    map.setView(initialCoordinates, 13);

    //should prob move this to backend and cache lol
    /*
    leaflet.tileLayer('https://{s}.tile.thunderforest.com/transport/{z}/{x}/{y}.png?apikey=99edd7abfb644a4dae152cd99ba43e74', {
        maxZoom: 19,
        minZoom: 7,
        subdomains: ['a', 'b', 'c'],
        attribution: 'Maps © <a href="https://www.thunderforest.com">Thunderforest</a>, Data © <a href="https://www.openstreetmap.org/copyright">OpenStreetMap contributors</a>'
    }).addTo(map);
    */

    leaflet.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        minZoom: 7,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    let heatmap_overlay_layer = leaflet.tileLayer('', {
        maxZoom: 19,
        minZoom: 7,
        opacity: 0.7,
    })

    return [map, heatmap_overlay_layer] as const
}

export function addStopMarker(stop: { stop_lat: number; stop_lon: number; }, map: Map) {
    leaflet.circleMarker([stop.stop_lat, stop.stop_lon]).addTo(map)
}

import { LatLng, type CircleMarkerOptions } from "leaflet";

interface StopMarkerData {
    stop_id: string
}

class StopMarker extends CircleMarker {
    public data: StopMarkerData;

    public constructor(latlng: LatLng, options: CircleMarkerOptions, data: StopMarkerData) {
        super(latlng, options);
        this.data = data;
    }

    public update_data(data: StopMarkerData) {
        this.data = data;
    }
}