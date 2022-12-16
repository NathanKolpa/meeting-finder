import L from 'leaflet';

export function showMeetingInfoPopup() {

}

export function loadMap() {
	const mapLayer = L.tileLayer('https://tile.mierune.co.jp/mierune_mono/{z}/{x}/{y}.png', {
		attribution: "Maptiles by <a href='http://mierune.co.jp/' target='_blank'>MIERUNE</a>, under CC BY. Data by <a href='http://osm.org/copyright' target='_blank'>OpenStreetMap</a> contributors, under ODbL."
	});

	const map = L.map('map', {
		center: [35.681, 139.767],
		zoom: 11,
		zoomControl: true,
		layers: [m_mono]
	});
}
