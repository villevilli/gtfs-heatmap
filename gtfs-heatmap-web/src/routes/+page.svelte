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
        let times: Record<string, any> = {};
        const leaflet = await import("leaflet");
        const module = await import("$lib/map");
        const maplib = module;

        let stops = fetch("http://localhost:8000/api/stops").then((response) =>
            response.json(),
        );

        let init_return = maplib.initOsmMap(mapElement, initialCoordinates);

        map = init_return[0];
        heatmaplayer = init_return[1];

        let stopMarkers: StopMarker[] = [];

        for (const stop of await stops) {
            let currentMarker = new StopMarker(
                latLng(parseFloat(stop.latitude), parseFloat(stop.longitude)),
                normalStyle,
                { stop_id: stop.id },
            ).addTo(map);

            currentMarker.on("click", markerClickListener);
            currentMarker.on("mouseover", markerHoverEnter);
            currentMarker.on("mouseout", markerHoverExit);

            stopMarkers.push(currentMarker);
        }

        function markerHoverEnter(event: LeafletEvent) {
            if (typeof times == "undefined") {
                return;
            }

            openPopup(event.target);
        }

        function markerHoverExit(event: LeafletEvent) {
            event.target.closePopup();
            event.target.unbindPopup();
        }

        function markerClickListener(event: LeafletEvent) {
            stopMarkers.forEach((marker) => marker.setStyle(normalStyle));
            focusMarker(event.target);
        }

        async function openPopup(stopmarker: StopMarker) {
            console.log(await times);

            const id = await stopmarker.data.stop_id;

            console.log(id);

            const newtimes = await times;

            stopmarker.bindPopup(`Time: ${newtimes[id]}`);
            stopmarker.openPopup();
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
            stop_id: string,
            selected_time: number,
            selected_day: String,
        ) {
            let date = new Date(Date.now());
            var day = date.getDay(),
                diff = date.getDate() - day;

            let date_epoch = date.setDate(diff);

            let dayNum;

            switch (selected_day) {
                case "Mon":
                    dayNum = 0;
                    break;
                case "Tue":
                    dayNum = 1;
                    break;
                case "Wen":
                    dayNum = 2;
                    break;
                case "Thu":
                    dayNum = 3;
                    break;
                case "Fri":
                    dayNum = 4;
                    break;
                case "Sat":
                    dayNum = 5;
                    break;
                case "Sun":
                    dayNum = 6;
                    break;
                default:
                    console.error("Day is not a valid day");
                    dayNum = 0;
            }

            date_epoch = Math.round(
                date_epoch / 1000 + selected_time * 3600 + dayNum * 86400,
            );

            times = fetch(
                `http://localhost:8000/api/stops/${stop_id}/dijkstras/${date_epoch}`,
            ).then((r) => r.json());
            layer.setUrl(
                `http://localhost:8000/api/tiles/${stop_id}/${selected_time}/${selected_day}/{z}/{x}/{y}/tile.webp`,
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
