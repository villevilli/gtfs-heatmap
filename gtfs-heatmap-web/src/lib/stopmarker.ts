import { CircleMarker, LatLng, type CircleMarkerOptions } from "leaflet";

export interface StopMarkerData {
    stop_id: String
}

export class StopMarker extends CircleMarker {
    public data: StopMarkerData;

    public constructor(latlng: LatLng, options: CircleMarkerOptions, data: StopMarkerData) {
        super(latlng, options);
        this.data = data;
    }

    public update_data(data: StopMarkerData) {
        this.data = data;
    }
}