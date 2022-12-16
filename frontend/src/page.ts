import L from 'leaflet';

export function showMeetingInfoPopup() {

}

function loadMap() {
    const mapLayer = L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        noWrap: true,
        attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    })

    const map = L.map('map', {
        zoomControl: true,
        zoom: 2,
        minZoom: 2,
        center: {lng: 0, lat: 0},
        layers: [mapLayer]
    });

    map.setMaxBounds(map.getBounds());
}

export function initialize() {
    loadMap();
}