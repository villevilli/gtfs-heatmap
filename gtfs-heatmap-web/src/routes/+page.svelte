<script lang="ts">
    import { onMount } from "svelte";
    import {
        type CircleMarkerOptions,
        type LatLngExpression,
        type LeafletEvent,
        type TileLayer,
        type Map,
        latLng,
    } from "leaflet";
    import { StopMarker } from "$lib/stopmarker";
    import "leaflet";

    let map: any;
    let heatmaplayer: TileLayer;
    let mapElement: HTMLElement;
    let selected_time = 12;
    let selected_day = "Mon";

    const initialCoordinates: LatLngExpression = [60.2, 25.0];

    const focusedStyle: CircleMarkerOptions = {
        color: "#ff9532",
        radius: 10,
    };
    const normalStyle: CircleMarkerOptions = {
        color: "#3388ff",
        radius: 7,
    };

    onMount(async () => {
        const leaflet = await import("leaflet");
        const module = await import("$lib/map");
        const maplib = module;

        let response = fetch("http://localhost:8000/api/stops");
        let init_return = maplib.initOsmMap(mapElement, initialCoordinates);

        map = init_return[0];
        heatmaplayer = init_return[1];

        let stops = await (await response).json();

        console.log(stops);

        let stopMarkers: StopMarker[] = [];

        for (const stop of stops) {
            console.log(stop);
            let currentMarker = new StopMarker(
                latLng(parseFloat(stop.stop_lat), parseFloat(stop.stop_lon)),
                normalStyle,
                { stop_id: stop.stop_id },
            ).addTo(map);

            currentMarker.on("click", markerClickListener);

            stopMarkers.push(currentMarker);
        }

        function markerClickListener(event: LeafletEvent) {
            stopMarkers.forEach((marker) => marker.setStyle(normalStyle));
            focusMarker(event.target);
        }

        function focusMarker(stopMarker: StopMarker) {
            stopMarker.setStyle(focusedStyle);
            stopMarker.bringToFront();
            updateHeatmap(
                map,
                heatmaplayer,
                stopMarker.data.stop_id,
                selected_time,
                selected_day,
            );
        }

        function updateHeatmap(
            map: Map,
            layer: TileLayer,
            stop_id: String,
            selected_time: Number,
            selected_day: String,
        ) {
            layer.setUrl(
                `http://localhost:8000/api/tiles/${stop_id}/${selected_time}/${selected_day}/{z}/{x}/{y}/tile.png`,
                false,
            );
            map.addLayer(layer);
        }
    });
</script>

<svelte:head>
    <link
        rel="stylesheet"
        href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
        integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
        crossorigin=""
    /></svelte:head
>

<div class="map" bind:this={mapElement} />

<div class="controlContainer leafletBar">
    <input
        id="time"
        type="range"
        min="0"
        max="24"
        step="2"
        bind:value={selected_time}
    />
    <label for="time">{selected_time}:00</label>
    <select bind:value={selected_day}>
        <option value="Mon">Monday</option>
        <option value="Tue">Tuesday</option>
        <option value="Wen">Wednesday</option>
        <option value="Thu">Thursday</option>
        <option value="Fri">Friday</option>
        <option value="Sat">Saturday</option>
        <option value="Sun">Sunday</option>
    </select>
</div>

<style lang="scss">
    :global(body, html) {
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        margin: 0;
    }

    .controlContainer * {
        margin: 5px;
        display: flex;
        justify-content: center;
        text-align: center;
    }

    .controlContainer {
        position: fixed;
        bottom: 20px;
        left: 20px;
        color: black;
        background-color: white;
        z-index: 1000;
        display: flex;
        padding: 10px;
        justify-content: space-between;
        border-radius: 5px;
        box-shadow: 0px 0px 6px 1px rgb(46, 46, 46);
    }

    .map {
        box-sizing: border-box;
        width: 100%;
        height: 100%;
    }
</style>
