import {ResultsList} from "./resultsList";
import {MeetingMap} from "./meetingMap";

export type PageComponents = [ResultsList, MeetingMap];

export function initialize(): PageComponents {
    return [
        new ResultsList('results', 'loading'),
        new MeetingMap('map')
    ]
}