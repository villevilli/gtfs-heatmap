<script lang="ts">
    import { onMount } from "svelte";
    import type {
        CircleMarker,
        CircleMarkerOptions,
        LatLngExpression,
        LeafletEvent,
        Path,
    } from "leaflet";

    let map;
    let mapElement: HTMLElement;

    const initialCoordinates: LatLngExpression = [60.2, 25.0];

    const focusedStyle: CircleMarkerOptions = {
        color: "#ff9532",
    };
    const normalStyle: CircleMarkerOptions = {
        color: "#3388ff",
    };

    onMount(async () => {
        const leaflet = await import("leaflet");
        const module = await import("$lib/map");
        const maplib = module;

        let response = fetch("http://localhost:8000/api/stops");
        map = maplib.initOsmMap(mapElement, initialCoordinates);

        let stops = await (await response).json();

        let stopMarkers: CircleMarker[] = [];

        for (const stop of stops) {
            let currentMarker = leaflet
                .circleMarker([
                    stop.coordinates.latitude,
                    stop.coordinates.longitude,
                ])
                .addTo(map);

            currentMarker.on("click", markerClickListener);

            stopMarkers.push(currentMarker);
        }

        function markerClickListener(event: LeafletEvent) {
            stopMarkers.forEach((marker) => marker.setStyle(normalStyle));
            focusMarker(event.target);
        }

        function focusMarker(circleMarker: CircleMarker) {
            circleMarker.setStyle(focusedStyle);
            circleMarker.bringToFront();
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

<div class="map" bind:this={mapElement}></div>

<style lang="scss">
    :global(body, html) {
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        margin: 0;
    }

    .map {
        box-sizing: border-box;
        width: 100%;
        height: 100%;
    }
</style>
