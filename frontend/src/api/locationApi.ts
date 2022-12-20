import {MeetingPosition} from "../models";

export async function fetchPositionByQuery(query: string): Promise<MeetingPosition | null> {
    const request = await fetch(`https://nominatim.openstreetmap.org/search?q=${encodeURIComponent(query)}&format=json`);
    const response = await request.json() as Place[];

    if (response.length == 0) {
        return null;
    }

    let place = response[0];

    return {
        latitude: parseFloat(place.lat),
        longitude: parseFloat(place.lon),
    }
}

export interface Place {
    place_id:     number;
    licence:      string;
    osm_type:     string;
    osm_id:       number;
    boundingbox:  string[];
    lat:          string;
    lon:          string;
    display_name: string;
    class:        string;
    type:         string;
    importance:   number;
    icon?:        string;
}
